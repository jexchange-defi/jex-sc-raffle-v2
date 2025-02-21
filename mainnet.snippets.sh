#!/bin/bash

set -ue

SCRIPT_DIR=$(dirname $0)
BYTECODE="${SCRIPT_DIR}/output-docker/jex-sc-raffle-v-2/jex-sc-raffle-v-2.wasm"
PROXY=https://gateway.multiversx.com
MVX_TOOLS_URL=https://tools.multiversx.com
GUARDIAN_ADDRESS=erd1cvt4665apw4wwmjervlncdnhags8hdpu3l9rpca8y4xtz6njmyfqxatz8n
PLAY_API_URL=https://play-api.multiversx.com
SC_ADDRESS=$(mxpy data load --key=address-mainnet)
CHAIN=1

source "${SCRIPT_DIR}/_common.snippets.sh"

deploy() {
    echo 'You are about to deploy SC on mainnet (Ctrl-C to abort)'
    read answer

    read -p "2FA code: " TWO_FA_CODE

    mxpy contract deploy --bytecode=${BYTECODE} \
        --keyfile=${1} --gas-limit=100000000 --outfile="deploy.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} \
        --guardian "${GUARDIAN_ADDRESS}" \
        --guardian-service-url "${MVX_TOOLS_URL}/guardian" \
        --guardian-2fa-code "${TWO_FA_CODE}" \
        --version 2 --options 2 \
        --recall-nonce --send || return

    SC_ADDRESS=$(cat deploy.interaction.json | jq -r .contractAddress)

    mxpy data store --key=address-mainnet --value=${SC_ADDRESS}

    echo ""
    echo "Smart contract address: ${SC_ADDRESS}"
}

upgrade() {
    echo 'You are about to upgrade current SC on mainnet (Ctrl-C to abort)'
    read answer

    mxpy contract upgrade --bytecode=${BYTECODE} --metadata-payable \
        --keyfile=${1} --gas-limit=100000000 --outfile="deploy.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send ${SC_ADDRESS} || return

    echo ""
    echo "Smart contract upgraded: ${SC_ADDRESS}"
}

verify() {
    mxpy contract verify "${SC_ADDRESS}" \
        --packaged-src=./output-docker/jex-sc-raffle-v-2/jex-sc-raffle-v-2-0.0.0.source.json \
        --verifier-url="${PLAY_API_URL}" \
        --docker-image="multiversx/sdk-rust-contract-builder:v8.0.1" \
        --keyfile=${1}
}

CMD=$1
shift

$CMD $*
