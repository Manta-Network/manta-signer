package main

/*
#cgo LDFLAGS: -L./lib -lzkp
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import "unsafe"


const TRANSACTION_TYPE_WITHDRAW = "Withdraw"
const TRANSACTION_TYPE_PRIVATE_TRANSFER = "Private transfer"

// Information that Signer UI displays during transaction validation
type TransactioBatchSummary struct {
	transactionType string
	value           string
	currencySymbol  string
	recipient       string
}

func getPrivateTransferBatchParamsValue(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_private_transfer_batch_params_value((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	valueString := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return valueString
}

func getPrivateTransferBatchParamsCurrencySymbol(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_private_transfer_batch_params_currency_symbol((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	currencySymbol := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return currencySymbol
}

func getPrivateTransferBatchParamsRecipient(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_private_transfer_batch_params_recipient((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	recipient := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return recipient
}

func getReclaimBatchParamsValue(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_reclaim_batch_params_value((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	valueString := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return valueString
}

func getReclaimBatchParamsCurrencySymbol(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_reclaim_batch_params_currency_symbol((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	currencySymbol := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return currencySymbol
}

func getReclaimBatchSummary(bytes []byte) TransactioBatchSummary {
	return TransactioBatchSummary{
		transactionType: TRANSACTION_TYPE_WITHDRAW,
		value:           getReclaimBatchParamsValue(bytes),
		currencySymbol:  getReclaimBatchParamsCurrencySymbol(bytes),
		recipient:       "your public wallet",
	 }
}

func getPrivateTransferBatchSummary(bytes []byte) TransactioBatchSummary {
 return TransactioBatchSummary{
	transactionType: TRANSACTION_TYPE_PRIVATE_TRANSFER,
	value:           getPrivateTransferBatchParamsValue(bytes),
	currencySymbol:  getPrivateTransferBatchParamsCurrencySymbol(bytes),
	recipient:       getPrivateTransferBatchParamsRecipient(bytes),
 }
}

func getTransactionBatchSummary(bytes []byte, transactionType string) TransactioBatchSummary {
	if (transactionType == TRANSACTION_TYPE_WITHDRAW) {
		return getReclaimBatchSummary(bytes)
	} else {
		return getPrivateTransferBatchSummary(bytes)
	}
}
