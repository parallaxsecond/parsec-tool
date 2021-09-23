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

delete_key() {
# $1 - key type
# $2 - key name
    KEY="$2"

    echo
    echo "- Deleting the $1 key"
    run_cmd $PARSEC_TOOL_CMD delete-key --key-name $KEY
    rm -f /tmp/${KEY}.*
}

create_key() {
# $1 - key type
# $2 - key name
    KEY="$2"

    echo
    echo "- Creating an $1 key and exporting its public part"
    type_lower=$(echo $1 | tr '[:upper:]' '[:lower:]')
    run_cmd $PARSEC_TOOL_CMD create-${type_lower}-key --key-name $KEY

    if ! run_cmd $PARSEC_TOOL_CMD list-keys | tee /dev/stderr | grep -q "$KEY"; then
        echo "Error: $KEY is not listed"
        EXIT_CODE=$(($EXIT_CODE+1))
    fi

    run_cmd $PARSEC_TOOL_CMD export-public-key --key-name $KEY >/tmp/${KEY}.pem
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

    test_rsa
    test_ecc
}

test_rsa() {
    KEY="anta-key-rsa"
    TEST_STR="$(date) Parsec decryption test"

    create_key "RSA" $KEY

    # If the key was successfully created and exported
    if [ -s /tmp/${KEY}.pem ]; then
        debug cat /tmp/${KEY}.pem

        echo
        echo "- Encrypting \"$TEST_STR\" string using openssl and the exported public key"

        # Encrypt TEST_STR with the public key and base64-encode the result
        echo -n "$TEST_STR" | \
            run_cmd $OPENSSL rsautl -encrypt -pubin -inkey /tmp/${KEY}.pem | \
            $OPENSSL base64 -A -out /tmp/${KEY}.enc
        debug cat /tmp/${KEY}.enc

        echo
        echo "- Using Parsec to decrypt the result:"
        encr_str="$(run_cmd $PARSEC_TOOL_CMD decrypt $(cat /tmp/${KEY}.enc) --key-name $KEY)"
        echo $encr_str
        if [ "$encr_str" != "$TEST_STR" ]; then
            echo "Error: The result is different from the initial string"
            EXIT_CODE=$(($EXIT_CODE+1))
        fi
    fi

    delete_key "RSA" $KEY
}

test_ecc() {
    KEY="anta-key-ecc"
    TEST_STR="$(date) Parsec signature test"

    create_key "ECC" $KEY

    # If the key was successfully created and exported
    if [ -s /tmp/${KEY}.pem ]; then
        debug cat /tmp/${KEY}.pem

        echo
        echo "- Signing \"$TEST_STR\" string using the created ECC key"
        run_cmd $PARSEC_TOOL_CMD sign "$TEST_STR" --key-name $KEY >/tmp/${KEY}.sign
        debug cat /tmp/${KEY}.sign

        echo
        echo "- Using openssl and the exported public ECC key to verify the signature"
        # Parsec-tool produces base64-encoded signatures. Let's decode it before verifing.
        $OPENSSL base64 -d -in /tmp/${KEY}.sign -out /tmp/${KEY}.bin
        echo -n "$TEST_STR" | \
            $OPENSSL dgst -sha256 -verify /tmp/${KEY}.pem -signature /tmp/${KEY}.bin
    fi

    delete_key "ECC" $KEY
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
run_cmd $PARSEC_TOOL list-providers 2>/dev/null|grep "^ID:"|grep -v "0x00"| \
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