package main

/*
#cgo LDFLAGS: -L./lib -lzkp
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import "unsafe"


type TransactioBatchSummary struct {
	transactionType string
	value           string
	denomination    string
	recipient       string
}

func getPrivateTransferBatchParamsValue(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_private_transfer_batch_params_value((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	res2 := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return res2
}

func getPrivateTransferBatchParamsCurrencySymbol(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_private_transfer_batch_params_currency_symbol((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	res2 := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return res2
}

func getPrivateTransferBatchParamsRecipient(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_private_transfer_batch_params_recipient((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	res2 := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return res2
}

func getReclaimBatchParamsValue(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_reclaim_batch_params_value((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	res2 := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return res2
}

func getReclaimBatchParamsCurrencySymbol(bytes []byte) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	res := C.get_reclaim_batch_params_currency_symbol((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef)
	if (res != 0) {
		return ""
	}
	res2 := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return res2
}

func getReclaimBatchSummary(bytes []byte) TransactioBatchSummary {
	return TransactioBatchSummary{
		transactionType: "Reclaim",
		value:           getReclaimBatchParamsValue(bytes),
		denomination:    getReclaimBatchParamsCurrencySymbol(bytes),
		recipient:       "your public wallet",
	 }
}

func getPrivateTransferBatchSummary(bytes []byte) TransactioBatchSummary {
 return TransactioBatchSummary{
	transactionType: "PrivateTransfer",
	value:           getPrivateTransferBatchParamsValue(bytes),
	denomination:    getPrivateTransferBatchParamsCurrencySymbol(bytes),
	recipient:       getPrivateTransferBatchParamsRecipient(bytes),
 }
}

func getTransactionBatchSummary(bytes []byte, transactionType string) TransactioBatchSummary {
	if (transactionType == "Reclaim") {
		return getReclaimBatchSummary(bytes)
	} else {
		return getPrivateTransferBatchSummary(bytes)
	}
}
