#!/usr/bin/env sh

# Copyright 2021 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0

# Run simple end-two-end Parsec tests using parsec-tool and openssl

ping_parsec() {
    echo "Checking Parsec service... "
    $PARSEC_TOOL ping
}

run_cmd() {
    "$@"
    EXIT_CODE=$(($EXIT_CODE+$?))
}

debug() {
    if [ -n "$PARSEC_TOOL_DEBUG" ]; then
        "$@"
    fi
}

MY_TMP=$(mktemp -d)

cleanup () {
    if [ -n "$MY_TMP" ]; then
      rm -rf -- "$MY_TMP"
    fi
}

trap cleanup EXIT

delete_key() {
# $1 - key type
# $2 - key name
    KEY="$2"

    echo
    echo "- Deleting the $1 key"
    run_cmd $PARSEC_TOOL_CMD delete-key --key-name $KEY
    rm -f ${MY_TMP}/${KEY}.*
}

create_key() {
# $1 - key type ("RSA" or "ECC")
# $2 - key name
# $3 - key usage ("CRYPT" or "SIGN"), only consulted if $1 == "RSA"
    KEY="$2"

    if [ "$3" = "SIGN" -a "$1" = "RSA" ]; then
        EXTRA_CREATE_KEY_ARGS="--for-signing"
    else
        EXTRA_CREATE_KEY_ARGS=""
    fi

    echo
    echo "- Creating an $1 key and exporting its public part"
    type_lower=$(echo $1 | tr '[:upper:]' '[:lower:]')
    run_cmd $PARSEC_TOOL_CMD create-${type_lower}-key --key-name $KEY $EXTRA_CREATE_KEY_ARGS

    if ! run_cmd $PARSEC_TOOL_CMD list-keys | tee /dev/stderr | grep -q "$KEY"; then
        echo "Error: $KEY is not listed"
        EXIT_CODE=$(($EXIT_CODE+1))
    fi

    run_cmd $PARSEC_TOOL_CMD export-public-key --key-name $KEY >${MY_TMP}/${KEY}.pem
}

test_crypto_provider() {
# $1 - provider ID

    PARSEC_TOOL_CMD="$PARSEC_TOOL -p $1"

    echo
    echo "- Test random number generation"
    if run_cmd $PARSEC_TOOL_CMD list-opcodes 2>/dev/null | grep -q "PsaGenerateRandom"; then
        run_cmd $PARSEC_TOOL_CMD generate-random --nbytes 10
    else
        echo "This provider doesn't support random number generation"
    fi

    test_encryption
    test_decryption
    test_signing "RSA"
    test_signing "ECC"
    test_csr "RSA"
    test_csr "ECC"
}

test_encryption() {
    KEY="anta-key-rsa-encrypt"
    TEST_STR="$(date) Parsec public key encryption test"

    create_key "RSA" $KEY "CRYPT"

    # If the key was successfully created and exported
    if [ -s ${MY_TMP}/${KEY}.pem ]; then
        debug cat ${MY_TMP}/${KEY}.pem

        echo
        echo "- Encrypting \"$TEST_STR\" string using Parsec public key encryption"

        # Encrypt TEST_STR with the public key using Parsec rather than openssl
        # (No need to base64 encode this, because parsec-tool already does it)
        run_cmd $PARSEC_TOOL_CMD encrypt --key-name $KEY "$TEST_STR" > ${MY_TMP}/${KEY}.enc

        echo
        echo "- Using Parsec to decrypt the result (with the private key):"
        run_cmd $PARSEC_TOOL_CMD decrypt $(cat ${MY_TMP}/${KEY}.enc) --key-name $KEY \
                >${MY_TMP}/${KEY}.enc_str
        cat ${MY_TMP}/${KEY}.enc_str
        if [ "$(cat ${MY_TMP}/${KEY}.enc_str)" != "$TEST_STR" ]; then
            echo "Error: The result is different from the initial string"
            EXIT_CODE=$(($EXIT_CODE+1))
        fi
    fi

    delete_key "RSA" $KEY
}

test_decryption() {
    KEY="anta-key-rsa-crypt"
    TEST_STR="$(date) Parsec decryption test"

    create_key "RSA" $KEY "CRYPT"

    # If the key was successfully created and exported
    if [ -s ${MY_TMP}/${KEY}.pem ]; then
        debug cat ${MY_TMP}/${KEY}.pem

        echo
        echo "- Encrypting \"$TEST_STR\" string using openssl and the exported public key"

        # Encrypt TEST_STR with the public key and base64-encode the result
        printf "$TEST_STR" >${MY_TMP}/${KEY}.test_str
        run_cmd $OPENSSL rsautl -encrypt -pubin -inkey ${MY_TMP}/${KEY}.pem \
                                -in ${MY_TMP}/${KEY}.test_str -out ${MY_TMP}/${KEY}.bin
        run_cmd $OPENSSL base64 -A -in ${MY_TMP}/${KEY}.bin -out ${MY_TMP}/${KEY}.enc
        debug cat ${MY_TMP}/${KEY}.enc

        echo
        echo "- Using Parsec to decrypt the result:"
        run_cmd $PARSEC_TOOL_CMD decrypt $(cat ${MY_TMP}/${KEY}.enc) --key-name $KEY \
                >${MY_TMP}/${KEY}.enc_str
        cat ${MY_TMP}/${KEY}.enc_str
        if [ "$(cat ${MY_TMP}/${KEY}.enc_str)" != "$TEST_STR" ]; then
            echo "Error: The result is different from the initial string"
            EXIT_CODE=$(($EXIT_CODE+1))
        fi
    fi

    delete_key "RSA" $KEY
}

test_signing() {
# $1 - key type ("RSA" or "ECC")
    KEY="anta-key-sign"
    TEST_STR="$(date) Parsec signature test"

    create_key $1 $KEY "SIGN"

    # If the key was successfully created and exported
    if [ -s ${MY_TMP}/${KEY}.pem ]; then
        debug cat ${MY_TMP}/${KEY}.pem

        echo
        echo "- Signing \"$TEST_STR\" string using the created $1 key"
        run_cmd $PARSEC_TOOL_CMD sign "$TEST_STR" --key-name $KEY >${MY_TMP}/${KEY}.sign
        debug cat ${MY_TMP}/${KEY}.sign

        echo
        echo "- Using openssl and the exported public $1 key to verify the signature"
        # Parsec-tool produces base64-encoded signatures. Let's decode it before verifing.
        run_cmd $OPENSSL base64 -d -a -A -in ${MY_TMP}/${KEY}.sign -out ${MY_TMP}/${KEY}.bin

        printf "$TEST_STR" >${MY_TMP}/${KEY}.test_str
        run_cmd $OPENSSL dgst -sha256 -verify ${MY_TMP}/${KEY}.pem \
                              -signature ${MY_TMP}/${KEY}.bin ${MY_TMP}/${KEY}.test_str
    fi

    delete_key $1 $KEY
}

test_csr() {
# $1 - key type ("RSA" or "ECC")
    KEY="anta-key-csr"
    TEST_CN="parallaxsecond.com"
    TEST_SAN="localhost"
    TEST_SERIAL="EZ4U2CIXL"

    # CSR creation needs a signing key.
    create_key $1 $KEY "SIGN"

    # If the key was successfully created and exported
    if [ -s ${MY_TMP}/${KEY}.pem ]; then
        debug cat ${MY_TMP}/${KEY}.pem

        echo
        echo "- Creating a certificate signing request (CSR) from the test key."
        run_cmd $PARSEC_TOOL_CMD create-csr --cn ${TEST_CN} --san ${TEST_SAN} --serialNumber ${TEST_SERIAL} --key-name $KEY >${MY_TMP}/${KEY}.csr
        debug cat ${MY_TMP}/${KEY}.csr

        echo
        echo "- Using openssl to inspect the CSR content and verify the public key."
        run_cmd $OPENSSL req -text -noout -verify -in ${MY_TMP}/${KEY}.csr >${MY_TMP}/${KEY}.txt
        debug cat ${MY_TMP}/${KEY}.txt

        if ! cat ${MY_TMP}/${KEY}.txt | grep "Subject:" | grep "serialNumber = ${TEST_SERIAL}"; then
            echo "Error: The CSR does not contain the serialNumber field of the Distinguished Name"
            EXIT_CODE=$(($EXIT_CODE+1))
        fi
    fi

    delete_key $1 $KEY
}

PARSEC_SERVICE_ENDPOINT="${PARSEC_SERVICE_ENDPOINT:-unix:/run/parsec/parsec.sock}"
PARSEC_TOOL="${PARSEC_TOOL:-$(which parsec-tool)}"
OPENSSL="${OPENSSL:-$(which openssl)}"

if [ -z "$PARSEC_TOOL" ] || [ -z "$OPENSSL" ]; then
    echo "ERROR: Cannot find either parsec-tool or openssl."
    echo "  Install the tools in PATH or define PARSEC_TOOL and OPENSSL variables"
    exit 1
fi

PARSEC_TOOL_DEBUG=
PROVIDER=
while [ "$#" -gt 0 ]; do
    case "$1" in
        -[0-9]* )
            PROVIDER=${1#-}
        ;;
        -d )
            PARSEC_TOOL_DEBUG="True"
            RUST_LOG="${RUST_LOG:-trace}"
            set -x
        ;;
        *)
            cat <<EOF
Usage: $0 [parameter]
  Parameters:
    -h:   Print help
    -d:   Debug output
    -N:   Test only the provider with N ID
EOF
            exit
        ;;
    esac
    shift
done

export RUST_LOG="${RUST_LOG:-info}"
if ! ping_parsec; then exit 1; fi

EXIT_CODE=0
run_cmd $PARSEC_TOOL list-providers 2>/dev/null | grep "^ID:" | grep -v "0x00" \
        >${MY_TMP}/providers.lst

exec < ${MY_TMP}/providers.lst
while IFS= read -r prv; do
    # Format of list-providers output:
    #ID: 0x01 (Mbed Crypto provider)
    #ID: 0x03 (TPM provider)
    prv_id=$(echo $prv | cut -f 2 -d ' ')
    prv_id=$(echo $(($prv_id))) # Hex -> decimal

    if [ -z "$PROVIDER" ] || [ "$PROVIDER" -eq "$prv_id" ]; then
        prv_name=${prv##*(}
        prv_name=${prv_name%)*}

        echo
        echo "Testing $prv_name"
        test_crypto_provider $prv_id
    fi
done

exit $EXIT_CODE
