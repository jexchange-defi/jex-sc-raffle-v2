##
# Info
##

echo "Proxy: ${PROXY}"
echo "SC address: ${SC_ADDRESS:-Not deployed}"

##
# Owner
##

setFeesReceiver() {
    KEYFILE=$1

    read -p "Receiver address: " FEES_RECEIVER

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=${KEYFILE} --gas-limit=5000000 \
        --function="setFeesReceiver" \
        --arguments "str:${FEES_RECEIVER}" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

setProtocolFeePercent() {
    KEYFILE=$1

    read -p "Fee percent (0-100): " FEE_PERCENT

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=${KEYFILE} --gas-limit=5000000 \
        --function="setProtocolFeePercent" \
        --arguments "str:${FEE_PERCENT}" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

setRaffleCreationFee() {
    KEYFILE=$1

    read -p "Fee (EGLD): " EGLD_FEE

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=${KEYFILE} --gas-limit=5000000 \
        --function="setRaffleCreationFee" \
        --arguments "str:${EGLD_FEE}" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

issueTicketCollection() {
    KEYFILE=$1

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=${KEYFILE} --gas-limit=5000000 \
        --function="issueTicketCollection" \
        --arguments "str:${EGLD_FEE}" \
        --value "5000000000000000" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

pause() {
    KEYFILE=$1

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=${KEYFILE} --gas-limit=5000000 \
        --function="pause" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

unpause() {
    KEYFILE=$1

    mxpy contract call ${SC_ADDRESS} --recall-nonce --pem=${KEYFILE} --gas-limit=5000000 \
        --function="unpause" \
        --proxy=${PROXY} --chain=${CHAIN} --send || return
}

##
# View
##
