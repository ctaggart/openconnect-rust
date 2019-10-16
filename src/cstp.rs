#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, linkage)]
extern crate libc;
extern "C" {
    pub type ssl3_buf_freelist_st;
    pub type evp_pkey_ctx_st;
    pub type engine_st;
    pub type ASN1_VALUE_st;
    pub type X509_VERIFY_PARAM_ID_st;
    pub type x509_crl_method_st;
    pub type stack_st_GENERAL_NAMES;
    pub type ISSUING_DIST_POINT_st;
    pub type AUTHORITY_KEYID_st;
    pub type NAME_CONSTRAINTS_st;
    pub type stack_st_GENERAL_NAME;
    pub type stack_st_DIST_POINT;
    pub type X509_POLICY_CACHE_st;
    pub type ec_key_st;
    pub type bignum_ctx;
    pub type bn_blinding_st;
    pub type evp_pkey_asn1_method_st;
    pub type X509_POLICY_TREE_st;
    pub type cert_st;
    pub type sess_cert_st;
    pub type ssl3_enc_method;
    pub type stack_st_OCSP_RESPID;
    pub type _pqueue;
    pub type internal_state;
    pub type oc_pcsc_ctx;
    /*
 * stoken.h - public libstoken library interface
 *
 * Copyright 2012 Kevin Cernekee <cernekee@gmail.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
    /* Before API version 1.3 (stoken 0.8) this macro didn't exist.
 * Somewhat ironic, that the API version check itself needs to be
 * conditionally used depending on the API version. A very simple way
 * for users to handle this with an approximately correct answer is
 *   #include <stoken.h>
 *   #ifndef STOKEN_CHECK_VER
 *   #define STOKEN_CHECK_VER(x,y) 0
 *   #endif
 */
    pub type stoken_ctx;
    pub type gss_ctx_id_struct;
    pub type gss_name_struct;
    pub type pxProxyFactory_;
    /*
 * Summary: string dictionary
 * Description: dictionary of reusable strings, just used to avoid allocation
 *         and freeing operations.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
    /*
 * The dictionary.
 */
    pub type _xmlDict;
    #[no_mangle]
    fn time(_: *mut time_t) -> time_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    /* !USE_ASCII */
    #[no_mangle]
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    /* Indicates version A of RuneLocale */
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    #[no_mangle]
    fn atol(_: *const libc::c_char) -> libc::c_long;
    /*
 * Copyright (c) 2018 Apple Computer, Inc. All rights reserved.
 *
 * @APPLE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this
 * file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_LICENSE_HEADER_END@
 */
    /*
 * This header is included from <stdlib.h>, so the contents of this file have
 * broad source compatibility and POSIX conformance implications.
 * Be cautious about what is included and declared here.
 */
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    /* *< library version string; useful to check dll version */
    /*-************************************
*  Tuning parameter
**************************************/
/* !
 * LZ4_MEMORY_USAGE :
 * Memory usage formula : N->2^N Bytes (examples : 10 -> 1KB; 12 -> 4KB ; 16 -> 64KB; 20 -> 1MB; etc.)
 * Increasing memory usage improves compression ratio.
 * Reduced memory usage may improve speed, thanks to better cache locality.
 * Default value is 14, for 16KB, which nicely fits into Intel x86 L1 cache
 */
    /*-************************************
*  Simple Functions
**************************************/
/* ! LZ4_compress_default() :
 *  Compresses 'srcSize' bytes from buffer 'src'
 *  into already allocated 'dst' buffer of size 'dstCapacity'.
 *  Compression is guaranteed to succeed if 'dstCapacity' >= LZ4_compressBound(srcSize).
 *  It also runs faster, so it's a recommended setting.
 *  If the function cannot compress 'src' into a more limited 'dst' budget,
 *  compression stops *immediately*, and the function result is zero.
 *  In which case, 'dst' content is undefined (invalid).
 *      srcSize : max supported value is LZ4_MAX_INPUT_SIZE.
 *      dstCapacity : size of buffer 'dst' (which must be already allocated)
 *     @return  : the number of bytes written into buffer 'dst' (necessarily <= dstCapacity)
 *                or 0 if compression fails
 * Note : This function is protected against buffer overflow scenarios (never writes outside 'dst' buffer, nor read outside 'source' buffer).
 */
    #[no_mangle]
    fn LZ4_compress_default(src: *const libc::c_char, dst: *mut libc::c_char,
                            srcSize: libc::c_int, dstCapacity: libc::c_int)
     -> libc::c_int;
    /* ! LZ4_decompress_safe() :
 *  compressedSize : is the exact complete size of the compressed block.
 *  dstCapacity : is the size of destination buffer (which must be already allocated), presumed an upper bound of decompressed size.
 * @return : the number of bytes decompressed into destination buffer (necessarily <= dstCapacity)
 *           If destination buffer is not large enough, decoding will stop and output an error code (negative value).
 *           If the source stream is detected malformed, the function will stop decoding and return a negative result.
 * Note 1 : This function is protected against malicious data packets :
 *          it will never writes outside 'dst' buffer, nor read outside 'source' buffer,
 *          even if the compressed block is maliciously modified to order the decoder to do these actions.
 *          In such case, the decoder stops immediately, and considers the compressed block malformed.
 * Note 2 : compressedSize and dstCapacity must be provided to the function, the compressed block does not contain them.
 *          The implementation is free to send / store / derive this information in whichever way is most beneficial.
 *          If there is a need for a different format which bundles together both compressed data and its metadata, consider looking at lz4frame.h instead.
 */
    #[no_mangle]
    fn LZ4_decompress_safe(src: *const libc::c_char, dst: *mut libc::c_char,
                           compressedSize: libc::c_int,
                           dstCapacity: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn openconnect_get_cstp_cipher(_: *mut openconnect_info)
     -> *const libc::c_char;
    #[no_mangle]
    fn ssl_nonblock_read(vpninfo: *mut openconnect_info,
                         buf: *mut libc::c_void, maxlen: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lzs_decompress(dst: *mut libc::c_uchar, dstlen: libc::c_int,
                      src: *const libc::c_uchar, srclen: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ssl_nonblock_write(vpninfo: *mut openconnect_info,
                          buf: *mut libc::c_void, buflen: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ka_stalled_action(ka: *mut keepalive_info, timeout: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn keepalive_action(ka: *mut keepalive_info, timeout: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn cstp_handshake(vpninfo: *mut openconnect_info, init: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn ssl_reconnect(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn lzs_compress(dst: *mut libc::c_uchar, dstlen: libc::c_int,
                    src: *const libc::c_uchar, srclen: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn openconnect_random(bytes: *mut libc::c_void, len: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn buf_alloc() -> *mut oc_text_buf;
    #[no_mangle]
    fn buf_error(buf: *mut oc_text_buf) -> libc::c_int;
    #[no_mangle]
    fn dump_buf(vpninfo: *mut openconnect_info, prefix: libc::c_char,
                buf: *mut libc::c_char);
    #[no_mangle]
    fn buf_free(buf: *mut oc_text_buf) -> libc::c_int;
    #[no_mangle]
    fn openconnect_open_https(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn free_optlist(opt: *mut oc_vpn_option);
    #[no_mangle]
    fn openconnect_close_https(vpninfo: *mut openconnect_info,
                               final_0: libc::c_int);
    #[no_mangle]
    fn http_common_headers(vpninfo: *mut openconnect_info,
                           buf: *mut oc_text_buf);
    #[no_mangle]
    static mut openconnect_version_str: *const libc::c_char;
    #[no_mangle]
    fn buf_append(buf: *mut oc_text_buf, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn gather_dtls_ciphers(vpninfo: *mut openconnect_info,
                           buf: *mut oc_text_buf, buf12: *mut oc_text_buf);
    #[no_mangle]
    fn free_split_routes(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn unhex(data: *const libc::c_char) -> libc::c_uchar;
    /* The application can compare zlibVersion and ZLIB_VERSION for consistency.
   If the first character differs, the library code actually used is not
   compatible with the zlib.h header file used by the application.  This check
   is automatically made by deflateInit and inflateInit.
 */
    /*
ZEXTERN int ZEXPORT deflateInit OF((z_streamp strm, int level));

     Initializes the internal stream state for compression.  The fields
   zalloc, zfree and opaque must be initialized before by the caller.  If
   zalloc and zfree are set to Z_NULL, deflateInit updates them to use default
   allocation functions.

     The compression level must be Z_DEFAULT_COMPRESSION, or between 0 and 9:
   1 gives best speed, 9 gives best compression, 0 gives no compression at all
   (the input data is simply copied a block at a time).  Z_DEFAULT_COMPRESSION
   requests a default compromise between speed and compression (currently
   equivalent to level 6).

     deflateInit returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_STREAM_ERROR if level is not a valid compression level, or
   Z_VERSION_ERROR if the zlib library version (zlib_version) is incompatible
   with the version assumed by the caller (ZLIB_VERSION).  msg is set to null
   if there is no error message.  deflateInit does not perform any compression:
   this will be done by deflate().
*/
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    /*
    deflate compresses as much data as possible, and stops when the input
  buffer becomes empty or the output buffer becomes full.  It may introduce
  some output latency (reading input without producing any output) except when
  forced to flush.

    The detailed semantics are as follows.  deflate performs one or both of the
  following actions:

  - Compress more input starting at next_in and update next_in and avail_in
    accordingly.  If not all input can be processed (because there is not
    enough room in the output buffer), next_in and avail_in are updated and
    processing will resume at this point for the next call of deflate().

  - Generate more output starting at next_out and update next_out and avail_out
    accordingly.  This action is forced if the parameter flush is non zero.
    Forcing flush frequently degrades the compression ratio, so this parameter
    should be set only when necessary.  Some output may be provided even if
    flush is zero.

    Before the call of deflate(), the application should ensure that at least
  one of the actions is possible, by providing more input and/or consuming more
  output, and updating avail_in or avail_out accordingly; avail_out should
  never be zero before the call.  The application can consume the compressed
  output when it wants, for example when the output buffer is full (avail_out
  == 0), or after each call of deflate().  If deflate returns Z_OK and with
  zero avail_out, it must be called again after making room in the output
  buffer because there might be more output pending. See deflatePending(),
  which can be used if desired to determine whether or not there is more ouput
  in that case.

    Normally the parameter flush is set to Z_NO_FLUSH, which allows deflate to
  decide how much data to accumulate before producing output, in order to
  maximize compression.

    If the parameter flush is set to Z_SYNC_FLUSH, all pending output is
  flushed to the output buffer and the output is aligned on a byte boundary, so
  that the decompressor can get all input data available so far.  (In
  particular avail_in is zero after the call if enough output space has been
  provided before the call.) Flushing may degrade compression for some
  compression algorithms and so it should be used only when necessary.  This
  completes the current deflate block and follows it with an empty stored block
  that is three bits plus filler bits to the next byte, followed by four bytes
  (00 00 ff ff).

    If flush is set to Z_PARTIAL_FLUSH, all pending output is flushed to the
  output buffer, but the output is not aligned to a byte boundary.  All of the
  input data so far will be available to the decompressor, as for Z_SYNC_FLUSH.
  This completes the current deflate block and follows it with an empty fixed
  codes block that is 10 bits long.  This assures that enough bytes are output
  in order for the decompressor to finish the block before the empty fixed
  codes block.

    If flush is set to Z_BLOCK, a deflate block is completed and emitted, as
  for Z_SYNC_FLUSH, but the output is not aligned on a byte boundary, and up to
  seven bits of the current block are held to be written as the next byte after
  the next deflate block is completed.  In this case, the decompressor may not
  be provided enough bits at this point in order to complete decompression of
  the data provided so far to the compressor.  It may need to wait for the next
  block to be emitted.  This is for advanced applications that need to control
  the emission of deflate blocks.

    If flush is set to Z_FULL_FLUSH, all output is flushed as with
  Z_SYNC_FLUSH, and the compression state is reset so that decompression can
  restart from this point if previous compressed data has been damaged or if
  random access is desired.  Using Z_FULL_FLUSH too often can seriously degrade
  compression.

    If deflate returns with avail_out == 0, this function must be called again
  with the same value of the flush parameter and more output space (updated
  avail_out), until the flush is complete (deflate returns with non-zero
  avail_out).  In the case of a Z_FULL_FLUSH or Z_SYNC_FLUSH, make sure that
  avail_out is greater than six to avoid repeated flush markers due to
  avail_out == 0 on return.

    If the parameter flush is set to Z_FINISH, pending input is processed,
  pending output is flushed and deflate returns with Z_STREAM_END if there was
  enough output space.  If deflate returns with Z_OK or Z_BUF_ERROR, this
  function must be called again with Z_FINISH and more output space (updated
  avail_out) but no more input data, until it returns with Z_STREAM_END or an
  error.  After deflate has returned Z_STREAM_END, the only possible operations
  on the stream are deflateReset or deflateEnd.

    Z_FINISH can be used in the first deflate call after deflateInit if all the
  compression is to be done in a single step.  In order to complete in one
  call, avail_out must be at least the value returned by deflateBound (see
  below).  Then deflate is guaranteed to return Z_STREAM_END.  If not enough
  output space is provided, deflate will not return Z_STREAM_END, and it must
  be called again as described above.

    deflate() sets strm->adler to the Adler-32 checksum of all input read
  so far (that is, total_in bytes).  If a gzip stream is being generated, then
  strm->adler will be the CRC-32 checksum of the input read so far.  (See
  deflateInit2 below.)

    deflate() may update strm->data_type if it can make a good guess about
  the input data type (Z_BINARY or Z_TEXT).  If in doubt, the data is
  considered binary.  This field is only for information purposes and does not
  affect the compression algorithm in any manner.

    deflate() returns Z_OK if some progress has been made (more input
  processed or more output produced), Z_STREAM_END if all input has been
  consumed and all output has been produced (only when flush is set to
  Z_FINISH), Z_STREAM_ERROR if the stream state was inconsistent (for example
  if next_in or next_out was Z_NULL or the state was inadvertently written over
  by the application), or Z_BUF_ERROR if no progress is possible (for example
  avail_in or avail_out was zero).  Note that Z_BUF_ERROR is not fatal, and
  deflate() can be called again with more input and more output space to
  continue compressing.
*/
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    /*
     All dynamically allocated data structures for this stream are freed.
   This function discards any unprocessed input and does not flush any pending
   output.

     deflateEnd returns Z_OK if success, Z_STREAM_ERROR if the
   stream state was inconsistent, Z_DATA_ERROR if the stream was freed
   prematurely (some input or output was discarded).  In the error case, msg
   may be set but then points to a static string (which must not be
   deallocated).
*/
    /*
ZEXTERN int ZEXPORT inflateInit OF((z_streamp strm));

     Initializes the internal stream state for decompression.  The fields
   next_in, avail_in, zalloc, zfree and opaque must be initialized before by
   the caller.  In the current version of inflate, the provided input is not
   read or consumed.  The allocation of a sliding window will be deferred to
   the first call of inflate (if the decompression does not complete on the
   first call).  If zalloc and zfree are set to Z_NULL, inflateInit updates
   them to use default allocation functions.

     inflateInit returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_VERSION_ERROR if the zlib library version is incompatible with the
   version assumed by the caller, or Z_STREAM_ERROR if the parameters are
   invalid, such as a null pointer to the structure.  msg is set to null if
   there is no error message.  inflateInit does not perform any decompression.
   Actual decompression will be done by inflate().  So next_in, and avail_in,
   next_out, and avail_out are unused and unchanged.  The current
   implementation of inflateInit() does not process any header information --
   that is deferred until inflate() is called.
*/
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    /*
    inflate decompresses as much data as possible, and stops when the input
  buffer becomes empty or the output buffer becomes full.  It may introduce
  some output latency (reading input without producing any output) except when
  forced to flush.

  The detailed semantics are as follows.  inflate performs one or both of the
  following actions:

  - Decompress more input starting at next_in and update next_in and avail_in
    accordingly.  If not all input can be processed (because there is not
    enough room in the output buffer), then next_in and avail_in are updated
    accordingly, and processing will resume at this point for the next call of
    inflate().

  - Generate more output starting at next_out and update next_out and avail_out
    accordingly.  inflate() provides as much output as possible, until there is
    no more input data or no more space in the output buffer (see below about
    the flush parameter).

    Before the call of inflate(), the application should ensure that at least
  one of the actions is possible, by providing more input and/or consuming more
  output, and updating the next_* and avail_* values accordingly.  If the
  caller of inflate() does not provide both available input and available
  output space, it is possible that there will be no progress made.  The
  application can consume the uncompressed output when it wants, for example
  when the output buffer is full (avail_out == 0), or after each call of
  inflate().  If inflate returns Z_OK and with zero avail_out, it must be
  called again after making room in the output buffer because there might be
  more output pending.

    The flush parameter of inflate() can be Z_NO_FLUSH, Z_SYNC_FLUSH, Z_FINISH,
  Z_BLOCK, or Z_TREES.  Z_SYNC_FLUSH requests that inflate() flush as much
  output as possible to the output buffer.  Z_BLOCK requests that inflate()
  stop if and when it gets to the next deflate block boundary.  When decoding
  the zlib or gzip format, this will cause inflate() to return immediately
  after the header and before the first block.  When doing a raw inflate,
  inflate() will go ahead and process the first block, and will return when it
  gets to the end of that block, or when it runs out of data.

    The Z_BLOCK option assists in appending to or combining deflate streams.
  To assist in this, on return inflate() always sets strm->data_type to the
  number of unused bits in the last byte taken from strm->next_in, plus 64 if
  inflate() is currently decoding the last block in the deflate stream, plus
  128 if inflate() returned immediately after decoding an end-of-block code or
  decoding the complete header up to just before the first byte of the deflate
  stream.  The end-of-block will not be indicated until all of the uncompressed
  data from that block has been written to strm->next_out.  The number of
  unused bits may in general be greater than seven, except when bit 7 of
  data_type is set, in which case the number of unused bits will be less than
  eight.  data_type is set as noted here every time inflate() returns for all
  flush options, and so can be used to determine the amount of currently
  consumed input in bits.

    The Z_TREES option behaves as Z_BLOCK does, but it also returns when the
  end of each deflate block header is reached, before any actual data in that
  block is decoded.  This allows the caller to determine the length of the
  deflate block header for later use in random access within a deflate block.
  256 is added to the value of strm->data_type when inflate() returns
  immediately after reaching the end of the deflate block header.

    inflate() should normally be called until it returns Z_STREAM_END or an
  error.  However if all decompression is to be performed in a single step (a
  single call of inflate), the parameter flush should be set to Z_FINISH.  In
  this case all pending input is processed and all pending output is flushed;
  avail_out must be large enough to hold all of the uncompressed data for the
  operation to complete.  (The size of the uncompressed data may have been
  saved by the compressor for this purpose.)  The use of Z_FINISH is not
  required to perform an inflation in one step.  However it may be used to
  inform inflate that a faster approach can be used for the single inflate()
  call.  Z_FINISH also informs inflate to not maintain a sliding window if the
  stream completes, which reduces inflate's memory footprint.  If the stream
  does not complete, either because not all of the stream is provided or not
  enough output space is provided, then a sliding window will be allocated and
  inflate() can be called again to continue the operation as if Z_NO_FLUSH had
  been used.

     In this implementation, inflate() always flushes as much output as
  possible to the output buffer, and always uses the faster approach on the
  first call.  So the effects of the flush parameter in this implementation are
  on the return value of inflate() as noted below, when inflate() returns early
  when Z_BLOCK or Z_TREES is used, and when inflate() avoids the allocation of
  memory for a sliding window when Z_FINISH is used.

     If a preset dictionary is needed after this call (see inflateSetDictionary
  below), inflate sets strm->adler to the Adler-32 checksum of the dictionary
  chosen by the compressor and returns Z_NEED_DICT; otherwise it sets
  strm->adler to the Adler-32 checksum of all output produced so far (that is,
  total_out bytes) and returns Z_OK, Z_STREAM_END or an error code as described
  below.  At the end of the stream, inflate() checks that its computed Adler-32
  checksum is equal to that saved by the compressor and returns Z_STREAM_END
  only if the checksum is correct.

    inflate() can decompress and check either zlib-wrapped or gzip-wrapped
  deflate data.  The header type is detected automatically, if requested when
  initializing with inflateInit2().  Any information contained in the gzip
  header is not retained unless inflateGetHeader() is used.  When processing
  gzip-wrapped deflate data, strm->adler32 is set to the CRC-32 of the output
  produced so far.  The CRC-32 is checked against the gzip trailer, as is the
  uncompressed length, modulo 2^32.

    inflate() returns Z_OK if some progress has been made (more input processed
  or more output produced), Z_STREAM_END if the end of the compressed data has
  been reached and all uncompressed output has been produced, Z_NEED_DICT if a
  preset dictionary is needed at this point, Z_DATA_ERROR if the input data was
  corrupted (input stream not conforming to the zlib format or incorrect check
  value, in which case strm->msg points to a string with a more specific
  error), Z_STREAM_ERROR if the stream structure was inconsistent (for example
  next_in or next_out was Z_NULL, or the state was inadvertently written over
  by the application), Z_MEM_ERROR if there was not enough memory, Z_BUF_ERROR
  if no progress was possible or if there was not enough room in the output
  buffer when Z_FINISH is used.  Note that Z_BUF_ERROR is not fatal, and
  inflate() can be called again with more input and more output space to
  continue decompressing.  If Z_DATA_ERROR is returned, the application may
  then call inflateSync() to look for a good compression block if a partial
  recovery of the data is to be attempted.
*/
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    /*
     Fine tune deflate's internal compression parameters.  This should only be
   used by someone who understands the algorithm used by zlib's deflate for
   searching for the best matching string, and even then only by the most
   fanatic optimizer trying to squeeze out the last compressed bit for their
   specific input data.  Read the deflate.c source code for the meaning of the
   max_lazy, good_length, nice_length, and max_chain parameters.

     deflateTune() can be called after deflateInit() or deflateInit2(), and
   returns Z_OK on success, or Z_STREAM_ERROR for an invalid deflate stream.
 */
    #[no_mangle]
    fn deflateBound(strm: z_streamp, sourceLen: uLong) -> uLong;
    #[no_mangle]
    fn adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn deflateInit2_(strm: z_streamp, level: libc::c_int, method: libc::c_int,
                     windowBits: libc::c_int, memLevel: libc::c_int,
                     strategy: libc::c_int, version: *const libc::c_char,
                     stream_size: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateInit2_(strm: z_streamp, windowBits: libc::c_int,
                     version: *const libc::c_char, stream_size: libc::c_int)
     -> libc::c_int;
    /* Look up MSGID in the DOMAINNAME message catalog for the current
   LC_MESSAGES locale.  */
    #[no_mangle]
    fn libintl_dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_socklen_t = __uint32_t;
/*
 * The rune type below is declared to be an ``int'' instead of the more natural
 * ``unsigned long'' or ``long''.  Two things are happening here.  It is not
 * unsigned so that EOF (-1) can be naturally assigned to it and used.  Also,
 * it looks like 10646 will be a 31 bit standard.  This means that if your
 * ints cannot hold 32 bits, you will be in trouble.  The reason an int was
 * chosen over a long is that the is*() and to*() routines take ints (says
 * ANSI C), but they use __darwin_ct_rune_t instead of int.  By changing it
 * here, you lose a bit of ANSI conformance, but your programs will still
 * work.
 *
 * NOTE: rune_t is not covered by ANSI nor other standards, and should not
 * be instantiated outside of lib/libc/locale.  Use wchar_t.  wchar_t and
 * rune_t must be the same type.  Also wint_t must be no narrower than
 * wchar_t, and should also be able to hold all members of the largest
 * character set plus one extra value (WEOF). wint_t must be at least 16 bits.
 */
/* ct_rune_t */
/*
 * mbstate_t is an opaque object to keep conversion state, during multibyte
 * stream conversions.  The content must not be referenced by user programs.
 */
/* for alignment */
/* mbstate_t */
/* ptr1 - ptr2 */
/* __GNUC__ */
/* sizeof() */
/* va_list */
/* wchar_t */
/* rune_t */
/* wint_t */
/* clock() */
/* socklen_t (duh) */
/* byte count or error */
pub type __darwin_time_t = libc::c_long;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* __darwin_size_t */
pub type size_t = __darwin_size_t;
/*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 * 
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
pub type uint64_t = libc::c_ulonglong;
/*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 * 
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
pub type uint32_t = libc::c_uint;
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* __darwin_uid_t */
pub type uid_t = __darwin_uid_t;
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* __darwin_gid_t */
pub type gid_t = __darwin_gid_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__int32_t; 32],
}
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* __darwin_time_t */
/* __darwin_suseconds_t */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
    /* and microseconds */
}
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* __darwin_time_t */
pub type time_t = __darwin_time_t;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *const libc::c_char)
                               -> __darwin_rune_t>,
    pub __sputrune: Option<unsafe extern "C" fn(_: __darwin_rune_t,
                                                _: *mut libc::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *mut libc::c_char)
                               -> libc::c_int>,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
/*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 * 
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
pub type uint8_t = libc::c_uchar;
/*
 * Copyright (c) 2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 * 
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 * 
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 * 
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 * 
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
pub type uint16_t = libc::c_ushort;
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* __uint8_t */
pub type sa_family_t = __uint8_t;
/*
 * Copyright (c) 2003-2012 Apple Inc. All rights reserved.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_START@
 *
 * This file contains Original Code and/or Modifications of Original Code
 * as defined in and that are subject to the Apple Public Source License
 * Version 2.0 (the 'License'). You may not use this file except in
 * compliance with the License. The rights granted to you under the License
 * may not be used to create, or enable the creation or redistribution of,
 * unlawful or unlicensed copies of an Apple operating system, or to
 * circumvent, violate, or enable the circumvention or violation of, any
 * terms of an Apple operating system software license agreement.
 *
 * Please obtain a copy of the License at
 * http://www.opensource.apple.com/apsl/ and read it before using this file.
 *
 * The Original Code and all software distributed under the License are
 * distributed on an 'AS IS' basis, WITHOUT WARRANTY OF ANY KIND, EITHER
 * EXPRESS OR IMPLIED, AND APPLE HEREBY DISCLAIMS ALL SUCH WARRANTIES,
 * INCLUDING WITHOUT LIMITATION, ANY WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE, QUIET ENJOYMENT OR NON-INFRINGEMENT.
 * Please see the License for the specific language governing rights and
 * limitations under the License.
 *
 * @APPLE_OSREFERENCE_LICENSE_HEADER_END@
 */
/* __darwin_socklen_t */
pub type socklen_t = __darwin_socklen_t;
/* __MSFILTERREQ_DEFINED */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: __uint8_t,
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_canonname: *mut libc::c_char,
    pub ai_addr: *mut sockaddr,
    pub ai_next: *mut addrinfo,
    /* next structure in linked list */
}
/* ***************************************************************************/
/* Authentication form processing */
/* char * fields are static (owned by XML parser) and don't need to be
   freed by the form handling code — except for value, which for TEXT
   and PASSWORD options is allocated by openconnect_set_option_value()
   when process_form() interacts with the user and must be freed. */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_form_opt {
    pub next: *mut oc_form_opt,
    pub type_0: libc::c_int,
    pub name: *mut libc::c_char,
    pub label: *mut libc::c_char,
    pub _value: *mut libc::c_char,
    pub flags: libc::c_uint,
    pub reserved: *mut libc::c_void,
}
/* All fields are static, owned by the XML parser */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_choice {
    pub name: *mut libc::c_char,
    pub label: *mut libc::c_char,
    pub auth_type: *mut libc::c_char,
    pub override_name: *mut libc::c_char,
    pub override_label: *mut libc::c_char,
    pub second_auth: libc::c_int,
    pub secondary_username: *mut libc::c_char,
    pub secondary_username_editable: libc::c_int,
    pub noaaa: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_form_opt_select {
    pub form: oc_form_opt,
    pub nr_choices: libc::c_int,
    pub choices: *mut *mut oc_choice,
}
/* All char * fields are static, owned by the XML parser */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_auth_form {
    pub banner: *mut libc::c_char,
    pub message: *mut libc::c_char,
    pub error: *mut libc::c_char,
    pub auth_id: *mut libc::c_char,
    pub method: *mut libc::c_char,
    pub action: *mut libc::c_char,
    pub opts: *mut oc_form_opt,
    pub authgroup_opt: *mut oc_form_opt_select,
    pub authgroup_selection: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_split_include {
    pub route: *const libc::c_char,
    pub next: *mut oc_split_include,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_ip_info {
    pub addr: *const libc::c_char,
    pub netmask: *const libc::c_char,
    pub addr6: *const libc::c_char,
    pub netmask6: *const libc::c_char,
    pub dns: [*const libc::c_char; 3],
    pub nbns: [*const libc::c_char; 3],
    pub domain: *const libc::c_char,
    pub proxy_pac: *const libc::c_char,
    pub mtu: libc::c_int,
    pub split_dns: *mut oc_split_include,
    pub split_includes: *mut oc_split_include,
    pub split_excludes: *mut oc_split_include,
    pub gateway_addr: *mut libc::c_char,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_vpn_option {
    pub option: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub next: *mut oc_vpn_option,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_stats {
    pub tx_pkts: uint64_t,
    pub tx_bytes: uint64_t,
    pub rx_pkts: uint64_t,
    pub rx_bytes: uint64_t,
}
/* ***************************************************************************/
/* Byte commands to write into the cmd_fd:
 *
 *  CANCEL closes network connections, logs off the session (cookie)
 *    and shuts down the tun device.
 *  PAUSE closes network connections and returns. The caller is expected
 *    to call openconnect_mainloop() again soon.
 *  DETACH closes network connections and shuts down the tun device.
 *    It is not legal to call openconnect_mainloop() again after this,
 *    but a new instance of openconnect can be started using the same
 *    cookie.
 *  STATS calls the stats_handler.
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct openconnect_info {
    pub proto: *const vpn_proto,
    pub ic_legacy_to_utf8: iconv_t,
    pub ic_utf8_to_legacy: iconv_t,
    pub redirect_url: *mut libc::c_char,
    pub redirect_type: libc::c_int,
    pub esp_hmac: libc::c_uchar,
    pub esp_enc: libc::c_uchar,
    pub esp_compr: libc::c_uchar,
    pub esp_replay_protect: uint32_t,
    pub esp_lifetime_bytes: uint32_t,
    pub esp_lifetime_seconds: uint32_t,
    pub esp_ssl_fallback: uint32_t,
    pub current_esp_in: libc::c_int,
    pub old_esp_maxseq: libc::c_int,
    pub esp_in: [esp; 2],
    pub esp_out: esp,
    pub enc_key_len: libc::c_int,
    pub hmac_key_len: libc::c_int,
    pub hmac_out_len: libc::c_int,
    pub esp_magic: uint32_t,
    pub tncc_fd: libc::c_int,
    pub csd_xmltag: *const libc::c_char,
    pub csd_nostub: libc::c_int,
    pub platname: *mut libc::c_char,
    pub mobile_platform_version: *mut libc::c_char,
    pub mobile_device_type: *mut libc::c_char,
    pub mobile_device_uniqueid: *mut libc::c_char,
    pub csd_token: *mut libc::c_char,
    pub csd_ticket: *mut libc::c_char,
    pub csd_stuburl: *mut libc::c_char,
    pub csd_starturl: *mut libc::c_char,
    pub csd_waiturl: *mut libc::c_char,
    pub csd_preurl: *mut libc::c_char,
    pub csd_scriptname: *mut libc::c_char,
    pub opaque_srvdata: *mut xmlNode,
    pub profile_url: *mut libc::c_char,
    pub profile_sha1: *mut libc::c_char,
    pub proxy_factory: *mut pxProxyFactory,
    pub proxy_type: *mut libc::c_char,
    pub proxy: *mut libc::c_char,
    pub proxy_port: libc::c_int,
    pub proxy_fd: libc::c_int,
    pub proxy_user: *mut libc::c_char,
    pub proxy_pass: *mut libc::c_char,
    pub proxy_close_during_auth: libc::c_int,
    pub retry_on_auth_fail: libc::c_int,
    pub try_http_auth: libc::c_int,
    pub http_auth: [http_auth_state; 4],
    pub proxy_auth: [http_auth_state; 4],
    pub localname: *mut libc::c_char,
    pub hostname: *mut libc::c_char,
    pub unique_hostname: *mut libc::c_char,
    pub port: libc::c_int,
    pub urlpath: *mut libc::c_char,
    pub cert_expire_warning: libc::c_int,
    pub cert: *mut libc::c_char,
    pub sslkey: *mut libc::c_char,
    pub cert_password: *mut libc::c_char,
    pub cafile: *mut libc::c_char,
    pub no_system_trust: libc::c_uint,
    pub xmlconfig: *const libc::c_char,
    pub xmlsha1: [libc::c_char; 41],
    pub authgroup: *mut libc::c_char,
    pub nopasswd: libc::c_int,
    pub xmlpost: libc::c_int,
    pub dtls_ciphers: *mut libc::c_char,
    pub dtls12_ciphers: *mut libc::c_char,
    pub csd_wrapper: *mut libc::c_char,
    pub no_http_keepalive: libc::c_int,
    pub dump_http_traffic: libc::c_int,
    pub token_mode: libc::c_int,
    pub token_bypassed: libc::c_int,
    pub token_tries: libc::c_int,
    pub token_time: time_t,
    pub stoken_ctx: *mut stoken_ctx,
    pub stoken_pin: *mut libc::c_char,
    pub stoken_concat_pin: libc::c_int,
    pub stoken_interval: libc::c_int,
    pub oath_secret: *mut libc::c_char,
    pub oath_secret_len: size_t,
    pub oath_hmac_alg: C2RustUnnamed_12,
    pub hotp_secret_format: C2RustUnnamed_11,
    pub pcsc: *mut oc_pcsc_ctx,
    pub yubikey_pwhash: [libc::c_uchar; 16],
    pub lock_token: openconnect_lock_token_vfn,
    pub unlock_token: openconnect_unlock_token_vfn,
    pub tok_cbdata: *mut libc::c_void,
    pub peer_cert: *mut libc::c_void,
    pub peer_cert_sha1_raw: [uint8_t; 20],
    pub peer_cert_sha256_raw: [uint8_t; 32],
    pub peer_cert_hash: *mut libc::c_char,
    pub cert_list_handle: *mut libc::c_void,
    pub cert_list_size: libc::c_int,
    pub cookie: *mut libc::c_char,
    pub cookies: *mut oc_vpn_option,
    pub cstp_options: *mut oc_vpn_option,
    pub dtls_options: *mut oc_vpn_option,
    pub script_env: *mut oc_vpn_option,
    pub csd_env: *mut oc_vpn_option,
    pub pfs: libc::c_uint,
    pub no_tls13: libc::c_uint,
    pub cert_x509: *mut X509,
    pub https_ctx: *mut SSL_CTX,
    pub https_ssl: *mut SSL,
    pub ttls_bio_meth: *mut BIO_METHOD,
    pub ttls_pushbuf: *mut oc_text_buf,
    pub ttls_eap_ident: uint8_t,
    pub ttls_recvbuf: *mut libc::c_uchar,
    pub ttls_recvpos: libc::c_int,
    pub ttls_recvlen: libc::c_int,
    pub pin_cache: *mut pin_cache,
    pub ssl_times: keepalive_info,
    pub owe_ssl_dpd_response: libc::c_int,
    pub deflate_pkt_size: libc::c_int,
    pub deflate_pkt: *mut pkt,
    pub pending_deflated_pkt: *mut pkt,
    pub current_ssl_pkt: *mut pkt,
    pub oncp_control_queue: pkt_q,
    pub oncp_rec_size: libc::c_int,
    pub cstp_pkt: *mut pkt,
    pub dtls_pkt: *mut pkt,
    pub tun_pkt: *mut pkt,
    pub pkt_trailer: libc::c_int,
    pub inflate_strm: z_stream,
    pub inflate_adler32: uint32_t,
    pub deflate_strm: z_stream,
    pub deflate_adler32: uint32_t,
    pub disable_ipv6: libc::c_int,
    pub reconnect_timeout: libc::c_int,
    pub reconnect_interval: libc::c_int,
    pub dtls_attempt_period: libc::c_int,
    pub new_dtls_started: time_t,
    pub dtls_ctx: *mut SSL_CTX,
    pub dtls_ssl: *mut SSL,
    pub cstp_cipher: *mut libc::c_char,
    pub dtls_state: libc::c_int,
    pub dtls_need_reconnect: libc::c_int,
    pub dtls_times: keepalive_info,
    pub dtls_session_id: [libc::c_uchar; 32],
    pub dtls_secret: [libc::c_uchar; 48],
    pub dtls_app_id: [libc::c_uchar; 32],
    pub dtls_app_id_size: libc::c_uint,
    pub ift_seq: uint32_t,
    pub cisco_dtls12: libc::c_int,
    pub dtls_cipher: *mut libc::c_char,
    pub vpnc_script: *mut libc::c_char,
    pub uid_csd_given: libc::c_int,
    pub uid_csd: uid_t,
    pub gid_csd: gid_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub use_tun_script: libc::c_int,
    pub script_tun: libc::c_int,
    pub ifname: *mut libc::c_char,
    pub cmd_ifname: *mut libc::c_char,
    pub reqmtu: libc::c_int,
    pub basemtu: libc::c_int,
    pub banner: *const libc::c_char,
    pub ip_info: oc_ip_info,
    pub cstp_basemtu: libc::c_int,
    pub idle_timeout: libc::c_int,
    pub _select_nfds: libc::c_int,
    pub _select_rfds: fd_set,
    pub _select_wfds: fd_set,
    pub _select_efds: fd_set,
    pub tun_fd: libc::c_int,
    pub ssl_fd: libc::c_int,
    pub dtls_fd: libc::c_int,
    pub dtls_tos_current: libc::c_int,
    pub dtls_pass_tos: libc::c_int,
    pub dtls_tos_proto: libc::c_int,
    pub dtls_tos_optname: libc::c_int,
    pub cmd_fd: libc::c_int,
    pub cmd_fd_write: libc::c_int,
    pub got_cancel_cmd: libc::c_int,
    pub got_pause_cmd: libc::c_int,
    pub cancel_type: libc::c_char,
    pub incoming_queue: pkt_q,
    pub outgoing_queue: pkt_q,
    pub max_qlen: libc::c_int,
    pub stats: oc_stats,
    pub stats_handler: openconnect_stats_vfn,
    pub peer_addrlen: socklen_t,
    pub peer_addr: *mut sockaddr,
    pub dtls_addr: *mut sockaddr,
    pub dtls_local_port: libc::c_int,
    pub req_compr: libc::c_int,
    pub cstp_compr: libc::c_int,
    pub dtls_compr: libc::c_int,
    pub is_dyndns: libc::c_int,
    pub useragent: *mut libc::c_char,
    pub version_string: *mut libc::c_char,
    pub quit_reason: *const libc::c_char,
    pub verbose: libc::c_int,
    pub cbdata: *mut libc::c_void,
    pub validate_peer_cert: openconnect_validate_peer_cert_vfn,
    pub write_new_config: openconnect_write_new_config_vfn,
    pub process_auth_form: openconnect_process_auth_form_vfn,
    pub progress: openconnect_progress_vfn,
    pub protect_socket: openconnect_protect_socket_vfn,
    pub getaddrinfo_override: openconnect_getaddrinfo_vfn,
    pub setup_tun: openconnect_setup_tun_vfn,
    pub reconnected: openconnect_reconnected_vfn,
    pub ssl_read: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                              _: *mut libc::c_char, _: size_t)
                             -> libc::c_int>,
    pub ssl_gets: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                              _: *mut libc::c_char, _: size_t)
                             -> libc::c_int>,
    pub ssl_write: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                               _: *mut libc::c_char,
                                               _: size_t) -> libc::c_int>,
    /* All strings are UTF-8. If operating in a legacy environment where
   nl_langinfo(CODESET) returns anything other than UTF-8, or on Windows,
   the library will take appropriate steps to convert back to the legacy
   character set (or UTF-16) for file handling and wherever else it is
   appropriate to do so. Library functions may (but probably don't yet)
   return -EILSEQ if passed invalid UTF-8 strings. */
    /* Unlike previous versions of openconnect, no functions will take ownership
   of the provided strings. */
    /* Provide environment variables to be set in the CSD trojan environment
   before spawning it. Some callers may need to set $TMPDIR, $PATH and
   other such things if not running from a standard UNIX-like environment.
   To ensure that a variable is unset, pass its name with value==NULL.
   To clear all settings and allow the CSD trojan to inherit an unmodified
   environment, call with name==NULL. */
}
/* This string is static, valid only while the connection lasts. If you
 * are going to cache this to remember which certs the user has accepted,
 * make sure you also store the host/port for which it was accepted and
 * don't just accept this cert from *anywhere*. Also use use the check
 * function below instead of manually comparing. When this function
 * returns a string which *doesn't* match the previously-stored hash
 * matched with openconnect_check_peer_cert_hash(), you should store
 * the new result from this function in place of the old. It means
 * we have upgraded to a better hash function. */
/* Check if the current peer certificate matches a hash previously
 * obtained from openconect_get_peer_cert_hash(). Clients should not
 * attempt to do this using strcmp() and the *current* result of
 * openconnect_get_peer_cert_hash() because it might use
 * a different hash function today. This function will get it right.
 * Returns 0 on match; 1 on mismatch, -errno on failure. */
/* The buffers returned by these two functions must be freed with
   openconnect_free_cert_info(), especially on Windows. */
/* Returns the length of the created DER output, in a newly-allocated buffer
   that will need to be freed by openconnect_free_cert_info(). */
/* Creates a list of all certs in the peer's chain, returning the
   number of certs in the chain (or <0 on error). Only valid inside the
   validate_peer_cert callback. The caller should free the chain,
   but should not modify the contents. */
/* Contains a comma-separated list of authentication methods to enabled.
   Currently supported: Negotiate,NTLM,Digest,Basic */
/* These are strictly cosmetic. The strings differ depending on
 * whether OpenSSL or GnuTLS is being used. And even depending on the
 * version of GnuTLS. Do *not* attempt to do anything meaningful based
 * on matching these strings; if you want to do something like that then
 * ask for an API that *does* offer you what you need. */
/* These return a descriptive string of the compression algorithm 
 * in use (LZS, LZ4, ...). If no compression then NULL is returned. */
/* Returns the IP address of the exact host to which the connection
 * was made. In --cookieonly mode or in any other scenario involving
 * a "two stage" connection, it is important to reconnect by IP because
 * the server side may be using DNS trickery for load balancing.
 *
 * If the IP address is unavailable due to the use of a proxy, this will
 * fall back to returning the DNS name. */
/* Returns the hostname parsed out of the server name URL. This is
 * intended to be used by the validate_peer_cert callback to check that
 * the certificate matches the server name. */
/* Some software tokens, such as HOTP tokens, include a counter which
 * needs to be stored in persistent storage.
 *
 * For such tokens, the lock function is first invoked to obtain a lock
 * on the storage because we're about to generate a new code. It is
 * permitted to call openconnect_set_token_mode() from the lock function,
 * if the token storage has been updated since it was first loaded. The
 * token mode must not change; only the token secret.
 *
 * The unlock function is called when a token code has been generated,
 * with a new token secret to be written to the persistent storage. The
 * secret will be in the same format as it was originally received by
 * openconnect_set_token_mode(). The new token may be NULL if an error
 * was encountered generating the code, in which case it is only
 * necessary for the callback function to unlock the storage.
 */
/* Legacy stoken-only function; do not use */
/* The size must be 41 bytes, since that's the size of a 20-byte SHA1
   represented as hex with a trailing NUL. */
/* call this function to disable the system trust from being used to
 * verify the server certificate. @val is a boolean value.
 *
 * For backwards compatibility reasons this is enabled by default.
 */
/* Valid choices are: "linux", "linux-64", "win", "mac-intel",
   "android", and "apple-ios". This also selects the corresponding CSD
   trojan binary. */
/* The returned structures are owned by the library and may be freed/replaced
   due to rekey or reconnect. Assume that once the mainloop starts, the
   pointers are no longer valid. For similar reasons, it is unsafe to call
   this function from another thread. */
/* If this is set, then openconnect_obtain_cookie() will abort and return
   failure if the file descriptor is readable. Typically a user may create
   a pair of pipes with the pipe(2) system call, hand the readable one to
   this function, and then write a byte to the other end if it ever wants
   to cancel the connection. This way, a multi-threaded UI (which will be
   running openconnect_obtain_cookie() in a separate thread since it blocks)
   has the ability to cancel that call, reap its thread and free the
   vpninfo structure (or retry). An 'fd' argument of -1 will render the
   cancellation mechanism inactive. */
/* Create a nonblocking pipe used to send cancellations and other commands
   to the library. This returns a file descriptor to the write side of
   the pipe. Both sides will be closed by openconnect_vpninfo_free().
   This replaces openconnect_set_cancel_fd(). */
/* Open CSTP connection; on success, IP information will be available. */
/* Create a tun device through the OS kernel (typical use case). Both
   strings are optional and can be NULL if desired. */
/* Pass traffic to a script program (no tun device). */
/* Caller will provide a file descriptor for the tunnel traffic. */
/* Optional call to enable DTLS on the connection. */
/* Start the main loop; exits if OC_CMD_CANCEL is received on cmd_fd or
   the remote site aborts. */
/* The first (privdata) argument to each of these functions is either
   the privdata argument provided to openconnect_vpninfo_new_with_cbdata(),
   or if that argument was NULL then it'll be the vpninfo itself. */
/* When the server's certificate fails validation via the normal means,
   this function is called with the offending certificate along with
   a textual reason for the failure (which may not be translated, if
   it comes directly from OpenSSL, but will be if it is rejected for
   "certificate does not match hostname", because that check is done
   in OpenConnect and *is* translated). The function shall return zero
   if the certificate is (or has in the past been) explicitly accepted
   by the user, and non-zero to abort the connection. */
/* On a successful connection, the server may provide us with a new XML
   configuration file. This contains the list of servers that can be
   chosen by the user to connect to, amongst other stuff that we mostly
   ignore. By "new", we mean that the SHA1 indicated by the server does
   not match the SHA1 set with the openconnect_set_xmlsha1() above. If
   they don't match, or openconnect_set_xmlsha1() has not been called,
   then the new XML is downloaded and this function is invoked. */
/* Handle an authentication form, requesting input from the user.
 * Return value:
 *  < 0, on error
 *  = 0, when form was parsed and POST required
 *  = 1, when response was cancelled by user
 */
/* Logging output which the user *may* want to see. */
/* Callback to allow binding a newly created socket's file descriptor to
   a specific interface, e.g. with SO_BINDTODEVICE. This tells the kernel
   not to route the traffic in question over the VPN tunnel. */
/* Callback for obtaining traffic stats via OC_CMD_STATS.
 */
/* SSL certificate capabilities. openconnect_has_pkcs11_support() means that we
   can accept PKCS#11 URLs in place of filenames, for the certificate and key. */
/* The OpenSSL TPM ENGINE stores keys in a PEM file labelled with the string
   -----BEGIN TSS KEY BLOB-----. */
/* Software token capabilities. */
/* Query and select from among supported protocols */
/* Callback for configuring the interface after MTU detection finishes. */
/* Callback for indicating that a TCP reconnection succeeded. */
pub type openconnect_reconnected_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type openconnect_setup_tun_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type openconnect_getaddrinfo_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: *const libc::c_char, _: *const addrinfo,
                                _: *mut *mut addrinfo) -> libc::c_int>;
pub type openconnect_protect_socket_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ()>;
pub type openconnect_progress_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int,
                                _: *const libc::c_char, _: ...) -> ()>;
pub type openconnect_process_auth_form_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut oc_auth_form)
               -> libc::c_int>;
pub type openconnect_write_new_config_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: libc::c_int) -> libc::c_int>;
pub type openconnect_validate_peer_cert_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char)
               -> libc::c_int>;
pub type openconnect_stats_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const oc_stats)
               -> ()>;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pkt_q {
    pub head: *mut pkt,
    pub tail: *mut *mut pkt,
    pub count: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pkt {
    pub len: libc::c_int,
    pub next: *mut pkt,
    pub c2rust_unnamed: C2RustUnnamed,
    pub data: [libc::c_uchar; 0],
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub esp: C2RustUnnamed_4,
    pub oncp: C2RustUnnamed_3,
    pub cstp: C2RustUnnamed_2,
    pub gpst: C2RustUnnamed_1,
    pub pulse: C2RustUnnamed_0,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub pad: [libc::c_uchar; 8],
    pub vendor: uint32_t,
    pub type_0: uint32_t,
    pub len: uint32_t,
    pub ident: uint32_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub pad: [libc::c_uchar; 8],
    pub hdr: [libc::c_uchar; 16],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub pad: [libc::c_uchar; 16],
    pub hdr: [libc::c_uchar; 8],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub pad: [libc::c_uchar; 2],
    pub rec: [libc::c_uchar; 2],
    pub kmp: [libc::c_uchar; 20],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub spi: uint32_t,
    pub seq: uint32_t,
    pub iv: [libc::c_uchar; 16],
    pub payload: [libc::c_uchar; 0],
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct keepalive_info {
    pub dpd: libc::c_int,
    pub keepalive: libc::c_int,
    pub rekey: libc::c_int,
    pub rekey_method: libc::c_int,
    pub last_rekey: time_t,
    pub last_tx: time_t,
    pub last_rx: time_t,
    pub last_dpd: time_t,
}
/* ====================================================================
 * Copyright (c) 1998-2001 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
pub type SSL = ssl_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_st {
    pub version: libc::c_int,
    pub type_0: libc::c_int,
    pub method: *const SSL_METHOD,
    pub rbio: *mut BIO,
    pub wbio: *mut BIO,
    pub bbio: *mut BIO,
    pub rwstate: libc::c_int,
    pub in_handshake: libc::c_int,
    pub handshake_func: Option<unsafe extern "C" fn(_: *mut SSL)
                                   -> libc::c_int>,
    pub server: libc::c_int,
    pub new_session: libc::c_int,
    pub quiet_shutdown: libc::c_int,
    pub shutdown: libc::c_int,
    pub state: libc::c_int,
    pub rstate: libc::c_int,
    pub init_buf: *mut BUF_MEM,
    pub init_msg: *mut libc::c_void,
    pub init_num: libc::c_int,
    pub init_off: libc::c_int,
    pub packet: *mut libc::c_uchar,
    pub packet_length: libc::c_uint,
    pub s2: *mut ssl2_state_st,
    pub s3: *mut ssl3_state_st,
    pub d1: *mut dtls1_state_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_void,
                                                  _: size_t, _: *mut SSL,
                                                  _: *mut libc::c_void)
                                 -> ()>,
    pub msg_callback_arg: *mut libc::c_void,
    pub hit: libc::c_int,
    pub param: *mut X509_VERIFY_PARAM,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub mac_flags: libc::c_int,
    pub enc_read_ctx: *mut EVP_CIPHER_CTX,
    pub read_hash: *mut EVP_MD_CTX,
    pub expand: *mut COMP_CTX,
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub cert: *mut cert_st,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub session: *mut SSL_SESSION,
    pub generate_session_id: GEN_SESSION_CB,
    pub verify_mode: libc::c_int,
    pub verify_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut X509_STORE_CTX)
                                    -> libc::c_int>,
    pub info_callback: Option<unsafe extern "C" fn(_: *const SSL,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub error: libc::c_int,
    pub error_code: libc::c_int,
    pub psk_client_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_char,
                                                         _: libc::c_uint,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub psk_server_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub ctx: *mut SSL_CTX,
    pub debug: libc::c_int,
    pub verify_result: libc::c_long,
    pub ex_data: CRYPTO_EX_DATA,
    pub client_CA: *mut stack_st_X509_NAME,
    pub references: libc::c_int,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub first_packet: libc::c_int,
    pub client_version: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub tlsext_debug_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_uchar,
                                                     _: libc::c_int,
                                                     _: *mut libc::c_void)
                                    -> ()>,
    pub tlsext_debug_arg: *mut libc::c_void,
    pub tlsext_hostname: *mut libc::c_char,
    pub servername_done: libc::c_int,
    pub tlsext_status_type: libc::c_int,
    pub tlsext_status_expected: libc::c_int,
    pub tlsext_ocsp_ids: *mut stack_st_OCSP_RESPID,
    pub tlsext_ocsp_exts: *mut X509_EXTENSIONS,
    pub tlsext_ocsp_resp: *mut libc::c_uchar,
    pub tlsext_ocsp_resplen: libc::c_int,
    pub tlsext_ticket_expected: libc::c_int,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_opaque_prf_input: *mut libc::c_void,
    pub tlsext_opaque_prf_input_len: size_t,
    pub tlsext_session_ticket: *mut TLS_SESSION_TICKET_EXT,
    pub tls_session_ticket_ext_cb: tls_session_ticket_ext_cb_fn,
    pub tls_session_ticket_ext_cb_arg: *mut libc::c_void,
    pub tls_session_secret_cb: tls_session_secret_cb_fn,
    pub tls_session_secret_cb_arg: *mut libc::c_void,
    pub initial_ctx: *mut SSL_CTX,
    pub next_proto_negotiated: *mut libc::c_uchar,
    pub next_proto_negotiated_len: libc::c_uchar,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
    pub srtp_profile: *mut SRTP_PROTECTION_PROFILE,
    pub tlsext_heartbeat: libc::c_uint,
    pub tlsext_hb_pending: libc::c_uint,
    pub tlsext_hb_seq: libc::c_uint,
    pub renegotiate: libc::c_int,
    pub srp_ctx: SRP_CTX,
    pub alpn_client_proto_list: *mut libc::c_uchar,
    pub alpn_client_proto_list_len: libc::c_uint,
    /* If placed in pkcs12.h, we end up with a circular depency with pkcs7.h */
    /* Nothing */
    /* Nothing */
    /* Callback types for crypto.h */
    /* def HEADER_OPENSSL_TYPES_H */
}
pub type SRP_CTX = srp_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct srp_ctx_st {
    pub SRP_cb_arg: *mut libc::c_void,
    pub TLS_ext_srp_username_callback: Option<unsafe extern "C" fn(_:
                                                                       *mut SSL,
                                                                   _:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
    pub SRP_verify_param_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int>,
    pub SRP_give_srp_client_pwd_callback: Option<unsafe extern "C" fn(_:
                                                                          *mut SSL,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> *mut libc::c_char>,
    pub login: *mut libc::c_char,
    pub N: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub s: *mut BIGNUM,
    pub B: *mut BIGNUM,
    pub A: *mut BIGNUM,
    pub a: *mut BIGNUM,
    pub b: *mut BIGNUM,
    pub v: *mut BIGNUM,
    pub info: *mut libc::c_char,
    pub strength: libc::c_int,
    pub srp_Mask: libc::c_ulong,
}
pub type BIGNUM = bignum_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type SRTP_PROTECTION_PROFILE = srtp_protection_profile_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct srtp_protection_profile_st {
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_SRTP_PROTECTION_PROFILE {
    pub stack: _STACK,
}
/* crypto/stack/stack.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
pub type _STACK = stack_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st {
    pub num: libc::c_int,
    pub data: *mut *mut libc::c_char,
    pub sorted: libc::c_int,
    pub num_alloc: libc::c_int,
    pub comp: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                          _: *const libc::c_void)
                         -> libc::c_int>,
}
pub type SSL_CTX = ssl_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_ctx_st {
    pub method: *const SSL_METHOD,
    pub cipher_list: *mut stack_st_SSL_CIPHER,
    pub cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    pub cert_store: *mut x509_store_st,
    pub sessions: *mut lhash_st_SSL_SESSION,
    pub session_cache_size: libc::c_ulong,
    pub session_cache_head: *mut ssl_session_st,
    pub session_cache_tail: *mut ssl_session_st,
    pub session_cache_mode: libc::c_int,
    pub session_timeout: libc::c_long,
    pub new_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_st,
                                                    _: *mut SSL_SESSION)
                                   -> libc::c_int>,
    pub remove_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_ctx_st,
                                                       _: *mut SSL_SESSION)
                                      -> ()>,
    pub get_session_cb: Option<unsafe extern "C" fn(_: *mut ssl_st,
                                                    _: *mut libc::c_uchar,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_int)
                                   -> *mut SSL_SESSION>,
    pub stats: C2RustUnnamed_8,
    pub references: libc::c_int,
    pub app_verify_callback: Option<unsafe extern "C" fn(_:
                                                             *mut X509_STORE_CTX,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int>,
    pub app_verify_arg: *mut libc::c_void,
    pub default_passwd_callback: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_char,
                                                             _: libc::c_int,
                                                             _: libc::c_int,
                                                             _:
                                                                 *mut libc::c_void)
                                            -> libc::c_int>,
    pub default_passwd_callback_userdata: *mut libc::c_void,
    pub client_cert_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                    _: *mut *mut X509,
                                                    _: *mut *mut EVP_PKEY)
                                   -> libc::c_int>,
    pub app_gen_cookie_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                       _: *mut libc::c_uchar,
                                                       _: *mut libc::c_uint)
                                      -> libc::c_int>,
    pub app_verify_cookie_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _: libc::c_uint)
                                         -> libc::c_int>,
    pub ex_data: CRYPTO_EX_DATA,
    pub rsa_md5: *const EVP_MD,
    pub md5: *const EVP_MD,
    pub sha1: *const EVP_MD,
    pub extra_certs: *mut stack_st_X509,
    pub comp_methods: *mut stack_st_SSL_COMP,
    pub info_callback: Option<unsafe extern "C" fn(_: *const SSL,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub client_CA: *mut stack_st_X509_NAME,
    pub options: libc::c_ulong,
    pub mode: libc::c_ulong,
    pub max_cert_list: libc::c_long,
    pub cert: *mut cert_st,
    pub read_ahead: libc::c_int,
    pub msg_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_void,
                                                  _: size_t, _: *mut SSL,
                                                  _: *mut libc::c_void)
                                 -> ()>,
    pub msg_callback_arg: *mut libc::c_void,
    pub verify_mode: libc::c_int,
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub default_verify_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _:
                                                                 *mut X509_STORE_CTX)
                                            -> libc::c_int>,
    pub generate_session_id: GEN_SESSION_CB,
    pub param: *mut X509_VERIFY_PARAM,
    pub quiet_shutdown: libc::c_int,
    pub max_send_fragment: libc::c_uint,
    pub client_cert_engine: *mut ENGINE,
    pub tlsext_servername_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> libc::c_int>,
    pub tlsext_servername_arg: *mut libc::c_void,
    pub tlsext_tick_key_name: [libc::c_uchar; 16],
    pub tlsext_tick_hmac_key: [libc::c_uchar; 16],
    pub tlsext_tick_aes_key: [libc::c_uchar; 16],
    pub tlsext_ticket_key_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _:
                                                              *mut EVP_CIPHER_CTX,
                                                          _: *mut HMAC_CTX,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
    pub tlsext_status_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                      _: *mut libc::c_void)
                                     -> libc::c_int>,
    pub tlsext_status_arg: *mut libc::c_void,
    pub tlsext_opaque_prf_input_callback: Option<unsafe extern "C" fn(_:
                                                                          *mut SSL,
                                                                      _:
                                                                          *mut libc::c_void,
                                                                      _:
                                                                          size_t,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> libc::c_int>,
    pub tlsext_opaque_prf_input_callback_arg: *mut libc::c_void,
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_client_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_char,
                                                         _: libc::c_uint,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub psk_server_callback: Option<unsafe extern "C" fn(_: *mut SSL,
                                                         _:
                                                             *const libc::c_char,
                                                         _:
                                                             *mut libc::c_uchar,
                                                         _: libc::c_uint)
                                        -> libc::c_uint>,
    pub freelist_max_len: libc::c_uint,
    pub wbuf_freelist: *mut ssl3_buf_freelist_st,
    pub rbuf_freelist: *mut ssl3_buf_freelist_st,
    pub srp_ctx: SRP_CTX,
    pub next_protos_advertised_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                               _:
                                                                   *mut *const libc::c_uchar,
                                                               _:
                                                                   *mut libc::c_uint,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int>,
    pub next_protos_advertised_cb_arg: *mut libc::c_void,
    pub next_proto_select_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                          _:
                                                              *mut *mut libc::c_uchar,
                                                          _:
                                                              *mut libc::c_uchar,
                                                          _:
                                                              *const libc::c_uchar,
                                                          _: libc::c_uint,
                                                          _:
                                                              *mut libc::c_void)
                                         -> libc::c_int>,
    pub next_proto_select_cb_arg: *mut libc::c_void,
    pub srtp_profiles: *mut stack_st_SRTP_PROTECTION_PROFILE,
    pub alpn_select_cb: Option<unsafe extern "C" fn(_: *mut SSL,
                                                    _:
                                                        *mut *const libc::c_uchar,
                                                    _: *mut libc::c_uchar,
                                                    _: *const libc::c_uchar,
                                                    _: libc::c_uint,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
    pub alpn_select_cb_arg: *mut libc::c_void,
    pub alpn_client_proto_list: *mut libc::c_uchar,
    pub alpn_client_proto_list_len: libc::c_uint,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
}
/* crypto/hmac/hmac.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* largest known is SHA512 */
pub type HMAC_CTX = hmac_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct hmac_ctx_st {
    pub md: *const EVP_MD,
    pub md_ctx: EVP_MD_CTX,
    pub i_ctx: EVP_MD_CTX,
    pub o_ctx: EVP_MD_CTX,
    pub key_length: libc::c_uint,
    pub key: [libc::c_uchar; 128],
}
pub type EVP_MD_CTX = env_md_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct env_md_ctx_st {
    pub digest: *const EVP_MD,
    pub engine: *mut ENGINE,
    pub flags: libc::c_ulong,
    pub md_data: *mut libc::c_void,
    pub pctx: *mut EVP_PKEY_CTX,
    pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                            _: *const libc::c_void, _: size_t)
                           -> libc::c_int>,
}
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type ENGINE = engine_st;
pub type EVP_MD = env_md_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct env_md_st {
    pub type_0: libc::c_int,
    pub pkey_type: libc::c_int,
    pub md_size: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX) -> libc::c_int>,
    pub update: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                            _: *const libc::c_void, _: size_t)
                           -> libc::c_int>,
    pub final_0: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                             _: *mut libc::c_uchar)
                            -> libc::c_int>,
    pub copy: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                          _: *const EVP_MD_CTX)
                         -> libc::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX)
                            -> libc::c_int>,
    pub sign: Option<unsafe extern "C" fn(_: libc::c_int,
                                          _: *const libc::c_uchar,
                                          _: libc::c_uint,
                                          _: *mut libc::c_uchar,
                                          _: *mut libc::c_uint,
                                          _: *mut libc::c_void)
                         -> libc::c_int>,
    pub verify: Option<unsafe extern "C" fn(_: libc::c_int,
                                            _: *const libc::c_uchar,
                                            _: libc::c_uint,
                                            _: *const libc::c_uchar,
                                            _: libc::c_uint,
                                            _: *mut libc::c_void)
                           -> libc::c_int>,
    pub required_pkey_type: [libc::c_int; 5],
    pub block_size: libc::c_int,
    pub ctx_size: libc::c_int,
    pub md_ctrl: Option<unsafe extern "C" fn(_: *mut EVP_MD_CTX,
                                             _: libc::c_int, _: libc::c_int,
                                             _: *mut libc::c_void)
                            -> libc::c_int>,
}
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct evp_cipher_ctx_st {
    pub cipher: *const EVP_CIPHER,
    pub engine: *mut ENGINE,
    pub encrypt: libc::c_int,
    pub buf_len: libc::c_int,
    pub oiv: [libc::c_uchar; 16],
    pub iv: [libc::c_uchar; 16],
    pub buf: [libc::c_uchar; 32],
    pub num: libc::c_int,
    pub app_data: *mut libc::c_void,
    pub key_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub cipher_data: *mut libc::c_void,
    pub final_used: libc::c_int,
    pub block_mask: libc::c_int,
    pub final_0: [libc::c_uchar; 32],
}
pub type EVP_CIPHER = evp_cipher_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct evp_cipher_st {
    pub nid: libc::c_int,
    pub block_size: libc::c_int,
    pub key_len: libc::c_int,
    pub iv_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                          _: *const libc::c_uchar,
                                          _: *const libc::c_uchar,
                                          _: libc::c_int) -> libc::c_int>,
    pub do_cipher: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                               _: *mut libc::c_uchar,
                                               _: *const libc::c_uchar,
                                               _: size_t) -> libc::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX)
                            -> libc::c_int>,
    pub ctx_size: libc::c_int,
    pub set_asn1_parameters: Option<unsafe extern "C" fn(_:
                                                             *mut EVP_CIPHER_CTX,
                                                         _: *mut ASN1_TYPE)
                                        -> libc::c_int>,
    pub get_asn1_parameters: Option<unsafe extern "C" fn(_:
                                                             *mut EVP_CIPHER_CTX,
                                                         _: *mut ASN1_TYPE)
                                        -> libc::c_int>,
    pub ctrl: Option<unsafe extern "C" fn(_: *mut EVP_CIPHER_CTX,
                                          _: libc::c_int, _: libc::c_int,
                                          _: *mut libc::c_void)
                         -> libc::c_int>,
    pub app_data: *mut libc::c_void,
}
/* crypto/asn1/asn1.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* let the recipient choose */
/* used in ASN1_TYPE */
/* used in ASN1 template code */
/* negative flag */
/* */
/* */
/* alias */
/* */
/* */
/* */
/* */
/* alias */
/* */
/* */
/* For use with d2i_ASN1_type_bytes() */
/* For use with ASN1_mbstring_copy() */
/* filled in by mkstack.pl */
/* nothing, no longer needed */
/*
 * We MUST make sure that, except for constness, asn1_ctx_st and
 * asn1_const_ctx are exactly the same.  Fortunately, as soon as the old ASN1
 * parsing macros are gone, we can throw this away as well...
 */
/* work char pointer */
/* end of sequence read for indefinite
                                 * encoding */
/* error code to use when returning an error */
/* constructed if 0x20, indefinite is 0x21 */
/* tag from last 'get object' */
/* class from last 'get object' */
/* length of last 'get object' */
/* largest value of p allowed */
/* temporary variable */
/* variable */
/* used in error processing */
/* work char pointer */
/* end of sequence read for indefinite
                                 * encoding */
/* error code to use when returning an error */
/* constructed if 0x20, indefinite is 0x21 */
/* tag from last 'get object' */
/* class from last 'get object' */
/* length of last 'get object' */
/* largest value of p allowed */
/* temporary variable */
/* variable */
/* used in error processing */
/*
 * These are used internally in the ASN1_OBJECT to keep track of whether the
 * names and data need to be free()ed
 */
/* internal use */
/* critical x509v3 object id */
/* internal use */
/* internal use */
/* data remains const after init */
/* Should we free this one */
/* Set if 0x07 has bits left value */
/*
 * This indicates that the ASN1_STRING is not a real value but just a place
 * holder for the location where indefinite length constructed data should be
 * inserted in the memory buffer
 */
/*
 * This flag is used by the CMS code to indicate that a string is not
 * complete and is a place holder for content when it had all been accessed.
 * The flag will be reset when content has been written to it.
 */
/*
 * This flag is used by ASN1 code to indicate an ASN1_STRING is an MSTRING
 * type.
 */
/* This is the base type that holds just about everything :-) */
/*
     * The value of the following field depends on the type being held.  It
     * is mostly being used for BIT_STRING so if the input data has a
     * non-zero 'unused bits' value, it will be handled correctly
     */
/*
 * ASN1_ENCODING structure: this is used to save the received encoding of an
 * ASN1 type. This is useful to get round problems with invalid encodings
 * which can break signatures.
 */
/* DER encoding */
/* Length of encoding */
/* set to 1 if 'enc' is invalid */
/* Used with ASN1 LONG type: if a long is set to this it is omitted */
/* size limits: this stuff is taken straight from RFC2459 */
/*
 * Declarations for template structures: for full definitions see asn1t.h
 */
/* This is just an opaque pointer */
/* Declare ASN1 functions: the implement macro in in asn1t.h */
/*-
 * The following macros and typedefs allow an ASN1_ITEM
 * to be embedded in a structure and referenced. Since
 * the ASN1_ITEM pointers need to be globally accessible
 * (possibly from shared libraries) they may exist in
 * different forms. On platforms that support it the
 * ASN1_ITEM structure itself will be globally exported.
 * Other platforms will export a function that returns
 * an ASN1_ITEM pointer.
 *
 * To handle both cases transparently the macros below
 * should be used instead of hard coding an ASN1_ITEM
 * pointer in a structure.
 *
 * The structure will look like this:
 *
 * typedef struct SOMETHING_st {
 *      ...
 *      ASN1_ITEM_EXP *iptr;
 *      ...
 * } SOMETHING;
 *
 * It would be initialised as e.g.:
 *
 * SOMETHING somevar = {...,ASN1_ITEM_ref(X509),...};
 *
 * and the actual pointer extracted with:
 *
 * const ASN1_ITEM *it = ASN1_ITEM_ptr(somevar.iptr);
 *
 * Finally an ASN1_ITEM pointer can be extracted from an
 * appropriate reference with: ASN1_ITEM_rptr(X509). This
 * would be used when a function takes an ASN1_ITEM * argument.
 *
 */
/* ASN1_ITEM pointer exported type */
/* Macro to obtain ASN1_ITEM pointer from exported type */
/* Macro to include ASN1_ITEM pointer from base type */
/* Parameters used by ASN1_STRING_print_ex() */
/*
 * These determine which characters to escape: RFC2253 special characters,
 * control characters and MSB set characters
 */
/*
 * This flag determines how we do escaping: normally RC2253 backslash only,
 * set this to use backslash and quote.
 */
/* These three flags are internal use only. */
/* Character is a valid PrintableString character */
/* Character needs escaping if it is the first character */
/* Character needs escaping if it is the last character */
/*
 * NB the internal flags are safely reused below by flags handled at the top
 * level.
 */
/*
 * If this is set we convert all character strings to UTF8 first
 */
/*
 * If this is set we don't attempt to interpret content: just assume all
 * strings are 1 byte per character. This will produce some pretty odd
 * looking output!
 */
/* If this is set we include the string type in the output */
/*
 * This determines which strings to display and which to 'dump' (hex dump of
 * content octets or DER encoding). We can only dump non character strings or
 * everything. If we don't dump 'unknown' they are interpreted as character
 * strings with 1 octet per character and are subject to the usual escaping
 * options.
 */
/*
 * These determine what 'dumping' does, we can dump the content octets or the
 * DER encoding: both use the RFC2253 #XXXXX notation.
 */
/*
 * All the string flags consistent with RFC2253, escaping control characters
 * isn't essential in RFC2253 but it is advisable anyway.
 */
pub type ASN1_TYPE = asn1_type_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct asn1_type_st {
    pub type_0: libc::c_int,
    pub value: C2RustUnnamed_5,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_5 {
    pub ptr: *mut libc::c_char,
    pub boolean: ASN1_BOOLEAN,
    pub asn1_string: *mut ASN1_STRING,
    pub object: *mut ASN1_OBJECT,
    pub integer: *mut ASN1_INTEGER,
    pub enumerated: *mut ASN1_ENUMERATED,
    pub bit_string: *mut ASN1_BIT_STRING,
    pub octet_string: *mut ASN1_OCTET_STRING,
    pub printablestring: *mut ASN1_PRINTABLESTRING,
    pub t61string: *mut ASN1_T61STRING,
    pub ia5string: *mut ASN1_IA5STRING,
    pub generalstring: *mut ASN1_GENERALSTRING,
    pub bmpstring: *mut ASN1_BMPSTRING,
    pub universalstring: *mut ASN1_UNIVERSALSTRING,
    pub utctime: *mut ASN1_UTCTIME,
    pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
    pub visiblestring: *mut ASN1_VISIBLESTRING,
    pub utf8string: *mut ASN1_UTF8STRING,
    pub set: *mut ASN1_STRING,
    pub sequence: *mut ASN1_STRING,
    pub asn1_value: *mut ASN1_VALUE,
}
pub type ASN1_VALUE = ASN1_VALUE_st;
pub type ASN1_STRING = asn1_string_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct asn1_string_st {
    pub length: libc::c_int,
    pub type_0: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub flags: libc::c_long,
}
pub type ASN1_UTF8STRING = asn1_string_st;
pub type ASN1_VISIBLESTRING = asn1_string_st;
pub type ASN1_GENERALIZEDTIME = asn1_string_st;
pub type ASN1_UTCTIME = asn1_string_st;
pub type ASN1_UNIVERSALSTRING = asn1_string_st;
pub type ASN1_BMPSTRING = asn1_string_st;
pub type ASN1_GENERALSTRING = asn1_string_st;
pub type ASN1_IA5STRING = asn1_string_st;
pub type ASN1_T61STRING = asn1_string_st;
pub type ASN1_PRINTABLESTRING = asn1_string_st;
pub type ASN1_OCTET_STRING = asn1_string_st;
pub type ASN1_BIT_STRING = asn1_string_st;
pub type ASN1_ENUMERATED = asn1_string_st;
pub type ASN1_INTEGER = asn1_string_st;
pub type ASN1_OBJECT = asn1_object_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct asn1_object_st {
    pub sn: *const libc::c_char,
    pub ln: *const libc::c_char,
    pub nid: libc::c_int,
    pub length: libc::c_int,
    pub data: *const libc::c_uchar,
    pub flags: libc::c_int,
}
pub type ASN1_BOOLEAN = libc::c_int;
/*
         * set and sequence are left complete and still contain the set or
         * sequence bytes
         */
/* crypto/x509/x509_vfy.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* number of paths to files or directories */
/* the list of paths or directories */
/* ******************************/
/*-
SSL_CTX -> X509_STORE
                -> X509_LOOKUP
                        ->X509_LOOKUP_METHOD
                -> X509_LOOKUP
                        ->X509_LOOKUP_METHOD

SSL     -> X509_STORE_CTX
                ->X509_STORE

The X509_STORE holds the tables etc for verification stuff.
A X509_STORE_CTX is used while validating a single certificate.
The X509_STORE has X509_LOOKUPs for looking up certs.
The X509_STORE then calls a function to actually verify the
certificate chain.
*/
/* one of the above types */
/* This is a static that defines the function interface */
/*
 * This structure hold all parameters associated with a verify operation by
 * including an X509_VERIFY_PARAM structure in related structures the
 * parameters used can be customized
 */
pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_VERIFY_PARAM_st {
    pub name: *mut libc::c_char,
    pub check_time: time_t,
    pub inh_flags: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub purpose: libc::c_int,
    pub trust: libc::c_int,
    pub depth: libc::c_int,
    pub policies: *mut stack_st_ASN1_OBJECT,
    pub id: *mut X509_VERIFY_PARAM_ID,
}
pub type X509_VERIFY_PARAM_ID = X509_VERIFY_PARAM_ID_st;
/* Time to use */
/* Inheritance flags */
/* Various verify flags */
/* purpose to check untrusted certificates */
/* trust setting to check */
/* Verify depth */
/* Permissible policies */
/* opaque ID data */
/* This is used to contain a list of bit names */
/* Macros for string operations */
/* for the is_set parameter to i2d_ASN1_SET */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_ASN1_OBJECT {
    pub stack: _STACK,
}
pub type GEN_SESSION_CB
    =
    Option<unsafe extern "C" fn(_: *const SSL, _: *mut libc::c_uchar,
                                _: *mut libc::c_uint) -> libc::c_int>;
pub type X509_STORE_CTX = x509_store_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_store_ctx_st {
    pub ctx: *mut X509_STORE,
    pub current_method: libc::c_int,
    pub cert: *mut X509,
    pub untrusted: *mut stack_st_X509,
    pub crls: *mut stack_st_X509_CRL,
    pub param: *mut X509_VERIFY_PARAM,
    pub other_ctx: *mut libc::c_void,
    pub verify: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                           -> libc::c_int>,
    pub verify_cb: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut X509_STORE_CTX)
                              -> libc::c_int>,
    pub get_issuer: Option<unsafe extern "C" fn(_: *mut *mut X509,
                                                _: *mut X509_STORE_CTX,
                                                _: *mut X509) -> libc::c_int>,
    pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509, _: *mut X509)
                                 -> libc::c_int>,
    pub check_revocation: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                     -> libc::c_int>,
    pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                             _: *mut *mut X509_CRL,
                                             _: *mut X509) -> libc::c_int>,
    pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                               _: *mut X509_CRL)
                              -> libc::c_int>,
    pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                              _: *mut X509_CRL, _: *mut X509)
                             -> libc::c_int>,
    pub check_policy: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                 -> libc::c_int>,
    pub lookup_certs: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_NAME)
                                 -> *mut stack_st_X509>,
    pub lookup_crls: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut X509_NAME)
                                -> *mut stack_st_X509_CRL>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                            -> libc::c_int>,
    pub valid: libc::c_int,
    pub last_untrusted: libc::c_int,
    pub chain: *mut stack_st_X509,
    pub tree: *mut X509_POLICY_TREE,
    pub explicit_policy: libc::c_int,
    pub error_depth: libc::c_int,
    pub error: libc::c_int,
    pub current_cert: *mut X509,
    pub current_issuer: *mut X509,
    pub current_crl: *mut X509_CRL,
    pub current_crl_score: libc::c_int,
    pub current_reasons: libc::c_uint,
    pub parent: *mut X509_STORE_CTX,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type CRYPTO_EX_DATA = crypto_ex_data_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct crypto_ex_data_st {
    pub sk: *mut stack_st_void,
    pub dummy: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_void {
    pub stack: _STACK,
}
pub type X509_CRL = X509_crl_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_crl_st {
    pub crl: *mut X509_CRL_INFO,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub akid: *mut AUTHORITY_KEYID,
    pub idp: *mut ISSUING_DIST_POINT,
    pub idp_flags: libc::c_int,
    pub idp_reasons: libc::c_int,
    pub crl_number: *mut ASN1_INTEGER,
    pub base_crl_number: *mut ASN1_INTEGER,
    pub sha1_hash: [libc::c_uchar; 20],
    pub issuers: *mut stack_st_GENERAL_NAMES,
    pub meth: *const X509_CRL_METHOD,
    pub meth_data: *mut libc::c_void,
}
pub type X509_CRL_METHOD = x509_crl_method_st;
pub type ISSUING_DIST_POINT = ISSUING_DIST_POINT_st;
pub type AUTHORITY_KEYID = AUTHORITY_KEYID_st;
pub type X509_ALGOR = X509_algor_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_algor_st {
    pub algorithm: *mut ASN1_OBJECT,
    pub parameter: *mut ASN1_TYPE,
}
pub type X509_CRL_INFO = X509_crl_info_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_crl_info_st {
    pub version: *mut ASN1_INTEGER,
    pub sig_alg: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub lastUpdate: *mut ASN1_TIME,
    pub nextUpdate: *mut ASN1_TIME,
    pub revoked: *mut stack_st_X509_REVOKED,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
pub type ASN1_ENCODING = ASN1_ENCODING_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ASN1_ENCODING_st {
    pub enc: *mut libc::c_uchar,
    pub len: libc::c_long,
    pub modified: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_EXTENSION {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_REVOKED {
    pub stack: _STACK,
}
pub type ASN1_TIME = asn1_string_st;
pub type X509_NAME = X509_name_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_name_st {
    pub entries: *mut stack_st_X509_NAME_ENTRY,
    pub modified: libc::c_int,
    pub bytes: *mut BUF_MEM,
    pub canon_enc: *mut libc::c_uchar,
    pub canon_enclen: libc::c_int,
}
pub type BUF_MEM = buf_mem_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_NAME_ENTRY {
    pub stack: _STACK,
}
pub type X509 = x509_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_st {
    pub cert_info: *mut X509_CINF,
    pub sig_alg: *mut X509_ALGOR,
    pub signature: *mut ASN1_BIT_STRING,
    pub valid: libc::c_int,
    pub references: libc::c_int,
    pub name: *mut libc::c_char,
    pub ex_data: CRYPTO_EX_DATA,
    pub ex_pathlen: libc::c_long,
    pub ex_pcpathlen: libc::c_long,
    pub ex_flags: libc::c_ulong,
    pub ex_kusage: libc::c_ulong,
    pub ex_xkusage: libc::c_ulong,
    pub ex_nscert: libc::c_ulong,
    pub skid: *mut ASN1_OCTET_STRING,
    pub akid: *mut AUTHORITY_KEYID,
    pub policy_cache: *mut X509_POLICY_CACHE,
    pub crldp: *mut stack_st_DIST_POINT,
    pub altname: *mut stack_st_GENERAL_NAME,
    pub nc: *mut NAME_CONSTRAINTS,
    pub sha1_hash: [libc::c_uchar; 20],
    pub aux: *mut X509_CERT_AUX,
}
pub type X509_CERT_AUX = x509_cert_aux_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_cert_aux_st {
    pub trust: *mut stack_st_ASN1_OBJECT,
    pub reject: *mut stack_st_ASN1_OBJECT,
    pub alias: *mut ASN1_UTF8STRING,
    pub keyid: *mut ASN1_OCTET_STRING,
    pub other: *mut stack_st_X509_ALGOR,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_ALGOR {
    pub stack: _STACK,
}
pub type NAME_CONSTRAINTS = NAME_CONSTRAINTS_st;
pub type X509_POLICY_CACHE = X509_POLICY_CACHE_st;
pub type X509_CINF = x509_cinf_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_cinf_st {
    pub version: *mut ASN1_INTEGER,
    pub serialNumber: *mut ASN1_INTEGER,
    pub signature: *mut X509_ALGOR,
    pub issuer: *mut X509_NAME,
    pub validity: *mut X509_VAL,
    pub subject: *mut X509_NAME,
    pub key: *mut X509_PUBKEY,
    pub issuerUID: *mut ASN1_BIT_STRING,
    pub subjectUID: *mut ASN1_BIT_STRING,
    pub extensions: *mut stack_st_X509_EXTENSION,
    pub enc: ASN1_ENCODING,
}
pub type X509_PUBKEY = X509_pubkey_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_pubkey_st {
    pub algor: *mut X509_ALGOR,
    pub public_key: *mut ASN1_BIT_STRING,
    pub pkey: *mut EVP_PKEY,
}
pub type EVP_PKEY = evp_pkey_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct evp_pkey_st {
    pub type_0: libc::c_int,
    pub save_type: libc::c_int,
    pub references: libc::c_int,
    pub ameth: *const EVP_PKEY_ASN1_METHOD,
    pub engine: *mut ENGINE,
    pub pkey: C2RustUnnamed_6,
    pub save_parameters: libc::c_int,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_ATTRIBUTE {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_6 {
    pub ptr: *mut libc::c_char,
    pub rsa: *mut rsa_st,
    pub dsa: *mut dsa_st,
    pub dh: *mut dh_st,
    pub ec: *mut ec_key_st,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dh_st {
    pub pad: libc::c_int,
    pub version: libc::c_int,
    pub p: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub length: libc::c_long,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub q: *mut BIGNUM,
    pub j: *mut BIGNUM,
    pub seed: *mut libc::c_uchar,
    pub seedlen: libc::c_int,
    pub counter: *mut BIGNUM,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DH_METHOD,
    pub engine: *mut ENGINE,
}
pub type DH_METHOD = dh_method;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dh_method {
    pub name: *const libc::c_char,
    pub generate_key: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
    pub compute_key: Option<unsafe extern "C" fn(_: *mut libc::c_uchar,
                                                 _: *const BIGNUM, _: *mut DH)
                                -> libc::c_int>,
    pub bn_mod_exp: Option<unsafe extern "C" fn(_: *const DH, _: *mut BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *mut BN_CTX,
                                                _: *mut BN_MONT_CTX)
                               -> libc::c_int>,
    pub init: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut DH) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub generate_params: Option<unsafe extern "C" fn(_: *mut DH,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut BN_GENCB)
                                    -> libc::c_int>,
}
pub type BN_GENCB = bn_gencb_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bn_gencb_st {
    pub ver: libc::c_uint,
    pub arg: *mut libc::c_void,
    pub cb: C2RustUnnamed_7,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_7 {
    pub cb_1: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: *mut libc::c_void) -> ()>,
    pub cb_2: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: *mut BN_GENCB) -> libc::c_int>,
}
pub type DH = dh_st;
pub type BN_MONT_CTX = bn_mont_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bn_mont_ctx_st {
    pub ri: libc::c_int,
    pub RR: BIGNUM,
    pub N: BIGNUM,
    pub Ni: BIGNUM,
    pub n0: [libc::c_ulong; 2],
    pub flags: libc::c_int,
}
pub type BN_CTX = bignum_ctx;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub write_params: libc::c_int,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub kinv: *mut BIGNUM,
    pub r: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DSA_METHOD,
    pub engine: *mut ENGINE,
}
pub type DSA_METHOD = dsa_method;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dsa_method {
    pub name: *const libc::c_char,
    pub dsa_do_sign: Option<unsafe extern "C" fn(_: *const libc::c_uchar,
                                                 _: libc::c_int, _: *mut DSA)
                                -> *mut DSA_SIG>,
    pub dsa_sign_setup: Option<unsafe extern "C" fn(_: *mut DSA,
                                                    _: *mut BN_CTX,
                                                    _: *mut *mut BIGNUM,
                                                    _: *mut *mut BIGNUM)
                                   -> libc::c_int>,
    pub dsa_do_verify: Option<unsafe extern "C" fn(_: *const libc::c_uchar,
                                                   _: libc::c_int,
                                                   _: *mut DSA_SIG,
                                                   _: *mut DSA)
                                  -> libc::c_int>,
    pub dsa_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA, _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BIGNUM,
                                                 _: *mut BN_CTX,
                                                 _: *mut BN_MONT_CTX)
                                -> libc::c_int>,
    pub bn_mod_exp: Option<unsafe extern "C" fn(_: *mut DSA, _: *mut BIGNUM,
                                                _: *mut BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *mut BN_CTX,
                                                _: *mut BN_MONT_CTX)
                               -> libc::c_int>,
    pub init: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub dsa_paramgen: Option<unsafe extern "C" fn(_: *mut DSA, _: libc::c_int,
                                                  _: *const libc::c_uchar,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_int,
                                                  _: *mut libc::c_ulong,
                                                  _: *mut BN_GENCB)
                                 -> libc::c_int>,
    pub dsa_keygen: Option<unsafe extern "C" fn(_: *mut DSA) -> libc::c_int>,
}
pub type DSA = dsa_st;
pub type DSA_SIG = DSA_SIG_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct DSA_SIG_st {
    pub r: *mut BIGNUM,
    pub s: *mut BIGNUM,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub meth: *const RSA_METHOD,
    pub engine: *mut ENGINE,
    pub n: *mut BIGNUM,
    pub e: *mut BIGNUM,
    pub d: *mut BIGNUM,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub dmp1: *mut BIGNUM,
    pub dmq1: *mut BIGNUM,
    pub iqmp: *mut BIGNUM,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub _method_mod_n: *mut BN_MONT_CTX,
    pub _method_mod_p: *mut BN_MONT_CTX,
    pub _method_mod_q: *mut BN_MONT_CTX,
    pub bignum_data: *mut libc::c_char,
    pub blinding: *mut BN_BLINDING,
    pub mt_blinding: *mut BN_BLINDING,
}
pub type BN_BLINDING = bn_blinding_st;
pub type RSA_METHOD = rsa_meth_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct rsa_meth_st {
    pub name: *const libc::c_char,
    pub rsa_pub_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_uchar,
                                                 _: *mut libc::c_uchar,
                                                 _: *mut RSA, _: libc::c_int)
                                -> libc::c_int>,
    pub rsa_pub_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_uchar,
                                                 _: *mut libc::c_uchar,
                                                 _: *mut RSA, _: libc::c_int)
                                -> libc::c_int>,
    pub rsa_priv_enc: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: *mut RSA, _: libc::c_int)
                                 -> libc::c_int>,
    pub rsa_priv_dec: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_uchar,
                                                  _: *mut libc::c_uchar,
                                                  _: *mut RSA, _: libc::c_int)
                                 -> libc::c_int>,
    pub rsa_mod_exp: Option<unsafe extern "C" fn(_: *mut BIGNUM,
                                                 _: *const BIGNUM,
                                                 _: *mut RSA, _: *mut BN_CTX)
                                -> libc::c_int>,
    pub bn_mod_exp: Option<unsafe extern "C" fn(_: *mut BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *const BIGNUM,
                                                _: *mut BN_CTX,
                                                _: *mut BN_MONT_CTX)
                               -> libc::c_int>,
    pub init: Option<unsafe extern "C" fn(_: *mut RSA) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut RSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub rsa_sign: Option<unsafe extern "C" fn(_: libc::c_int,
                                              _: *const libc::c_uchar,
                                              _: libc::c_uint,
                                              _: *mut libc::c_uchar,
                                              _: *mut libc::c_uint,
                                              _: *const RSA) -> libc::c_int>,
    pub rsa_verify: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: *const libc::c_uchar,
                                                _: libc::c_uint,
                                                _: *const libc::c_uchar,
                                                _: libc::c_uint,
                                                _: *const RSA)
                               -> libc::c_int>,
    pub rsa_keygen: Option<unsafe extern "C" fn(_: *mut RSA, _: libc::c_int,
                                                _: *mut BIGNUM,
                                                _: *mut BN_GENCB)
                               -> libc::c_int>,
}
pub type RSA = rsa_st;
pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
pub type X509_VAL = X509_val_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct X509_val_st {
    pub notBefore: *mut ASN1_TIME,
    pub notAfter: *mut ASN1_TIME,
}
pub type X509_POLICY_TREE = X509_POLICY_TREE_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509 {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_CRL {
    pub stack: _STACK,
}
pub type X509_STORE = x509_store_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct x509_store_st {
    pub cache: libc::c_int,
    pub objs: *mut stack_st_X509_OBJECT,
    pub get_cert_methods: *mut stack_st_X509_LOOKUP,
    pub param: *mut X509_VERIFY_PARAM,
    pub verify: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                           -> libc::c_int>,
    pub verify_cb: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut X509_STORE_CTX)
                              -> libc::c_int>,
    pub get_issuer: Option<unsafe extern "C" fn(_: *mut *mut X509,
                                                _: *mut X509_STORE_CTX,
                                                _: *mut X509) -> libc::c_int>,
    pub check_issued: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509, _: *mut X509)
                                 -> libc::c_int>,
    pub check_revocation: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                                     -> libc::c_int>,
    pub get_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                             _: *mut *mut X509_CRL,
                                             _: *mut X509) -> libc::c_int>,
    pub check_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                               _: *mut X509_CRL)
                              -> libc::c_int>,
    pub cert_crl: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                              _: *mut X509_CRL, _: *mut X509)
                             -> libc::c_int>,
    pub lookup_certs: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                  _: *mut X509_NAME)
                                 -> *mut stack_st_X509>,
    pub lookup_crls: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX,
                                                 _: *mut X509_NAME)
                                -> *mut stack_st_X509_CRL>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut X509_STORE_CTX)
                            -> libc::c_int>,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_LOOKUP {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_OBJECT {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_X509_NAME {
    pub stack: _STACK,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_SSL_COMP {
    pub stack: _STACK,
}
/* crypto/pem/pem.h */
/* Copyright (C) 1995-1997 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/*
   * Note that this structure is initialised by PEM_SealInit and cleaned up
   * by PEM_SealFinal (at least for now)
   */
/* enc_type is one off */
/*      char iv[8]; unused and wrong size */
/* what type of object */
/*-
        unused, and wrong size
        unsigned char iv[8]; */
/*-
    XXX(ben): don#t think this is used!
        STACK *x509_chain;      / * certificate chain */
/* signature type */
/* is the md encrypted or not? */
/* length of md_data */
/* message digest, could be pkey encrypted */
/* date encryption cipher */
/* key length */
/* key */
/*-
    unused, and wrong size
    unsigned char iv[8]; */
/* is the data encrypted */
/*
 * These macros make the PEM_read/PEM_write functions easier to maintain and
 * write. Now they are all implemented with either: IMPLEMENT_PEM_rw(...) or
 * IMPLEMENT_PEM_rw_cb(...)
 */
/* These are the same except they are for the declarations */
/* "userdata": new with OpenSSL 0.9.4 */
pub type pem_password_cb
    =
    unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int,
                         _: *mut libc::c_void) -> libc::c_int;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub sess_connect: libc::c_int,
    pub sess_connect_renegotiate: libc::c_int,
    pub sess_connect_good: libc::c_int,
    pub sess_accept: libc::c_int,
    pub sess_accept_renegotiate: libc::c_int,
    pub sess_accept_good: libc::c_int,
    pub sess_miss: libc::c_int,
    pub sess_timeout: libc::c_int,
    pub sess_cache_full: libc::c_int,
    pub sess_hit: libc::c_int,
    pub sess_cb_hit: libc::c_int,
}
pub type SSL_SESSION = ssl_session_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_session_st {
    pub ssl_version: libc::c_int,
    pub key_arg_length: libc::c_uint,
    pub key_arg: [libc::c_uchar; 8],
    pub master_key_length: libc::c_int,
    pub master_key: [libc::c_uchar; 48],
    pub session_id_length: libc::c_uint,
    pub session_id: [libc::c_uchar; 32],
    pub sid_ctx_length: libc::c_uint,
    pub sid_ctx: [libc::c_uchar; 32],
    pub psk_identity_hint: *mut libc::c_char,
    pub psk_identity: *mut libc::c_char,
    pub not_resumable: libc::c_int,
    pub sess_cert: *mut sess_cert_st,
    pub peer: *mut X509,
    pub verify_result: libc::c_long,
    pub references: libc::c_int,
    pub timeout: libc::c_long,
    pub time: libc::c_long,
    pub compress_meth: libc::c_uint,
    pub cipher: *const SSL_CIPHER,
    pub cipher_id: libc::c_ulong,
    pub ciphers: *mut stack_st_SSL_CIPHER,
    pub ex_data: CRYPTO_EX_DATA,
    pub prev: *mut ssl_session_st,
    pub next: *mut ssl_session_st,
    pub tlsext_hostname: *mut libc::c_char,
    pub tlsext_ecpointformatlist_length: size_t,
    pub tlsext_ecpointformatlist: *mut libc::c_uchar,
    pub tlsext_ellipticcurvelist_length: size_t,
    pub tlsext_ellipticcurvelist: *mut libc::c_uchar,
    pub tlsext_tick: *mut libc::c_uchar,
    pub tlsext_ticklen: size_t,
    pub tlsext_tick_lifetime_hint: libc::c_long,
    pub srp_username: *mut libc::c_char,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct stack_st_SSL_CIPHER {
    pub stack: _STACK,
}
pub type SSL_CIPHER = ssl_cipher_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_cipher_st {
    pub valid: libc::c_int,
    pub name: *const libc::c_char,
    pub id: libc::c_ulong,
    pub algorithm_mkey: libc::c_ulong,
    pub algorithm_auth: libc::c_ulong,
    pub algorithm_enc: libc::c_ulong,
    pub algorithm_mac: libc::c_ulong,
    pub algorithm_ssl: libc::c_ulong,
    pub algo_strength: libc::c_ulong,
    pub algorithm2: libc::c_ulong,
    pub strength_bits: libc::c_int,
    pub alg_bits: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct lhash_st_SSL_SESSION {
    pub dummy: libc::c_int,
}
pub type SSL_METHOD = ssl_method_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_method_st {
    pub version: libc::c_int,
    pub ssl_new: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
    pub ssl_clear: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
    pub ssl_free: Option<unsafe extern "C" fn(_: *mut SSL) -> ()>,
    pub ssl_accept: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
    pub ssl_connect: Option<unsafe extern "C" fn(_: *mut SSL) -> libc::c_int>,
    pub ssl_read: Option<unsafe extern "C" fn(_: *mut SSL,
                                              _: *mut libc::c_void,
                                              _: libc::c_int) -> libc::c_int>,
    pub ssl_peek: Option<unsafe extern "C" fn(_: *mut SSL,
                                              _: *mut libc::c_void,
                                              _: libc::c_int) -> libc::c_int>,
    pub ssl_write: Option<unsafe extern "C" fn(_: *mut SSL,
                                               _: *const libc::c_void,
                                               _: libc::c_int)
                              -> libc::c_int>,
    pub ssl_shutdown: Option<unsafe extern "C" fn(_: *mut SSL)
                                 -> libc::c_int>,
    pub ssl_renegotiate: Option<unsafe extern "C" fn(_: *mut SSL)
                                    -> libc::c_int>,
    pub ssl_renegotiate_check: Option<unsafe extern "C" fn(_: *mut SSL)
                                          -> libc::c_int>,
    pub ssl_get_message: Option<unsafe extern "C" fn(_: *mut SSL,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_long,
                                                     _: *mut libc::c_int)
                                    -> libc::c_long>,
    pub ssl_read_bytes: Option<unsafe extern "C" fn(_: *mut SSL,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_uchar,
                                                    _: libc::c_int,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub ssl_write_bytes: Option<unsafe extern "C" fn(_: *mut SSL,
                                                     _: libc::c_int,
                                                     _: *const libc::c_void,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub ssl_dispatch_alert: Option<unsafe extern "C" fn(_: *mut SSL)
                                       -> libc::c_int>,
    pub ssl_ctrl: Option<unsafe extern "C" fn(_: *mut SSL, _: libc::c_int,
                                              _: libc::c_long,
                                              _: *mut libc::c_void)
                             -> libc::c_long>,
    pub ssl_ctx_ctrl: Option<unsafe extern "C" fn(_: *mut SSL_CTX,
                                                  _: libc::c_int,
                                                  _: libc::c_long,
                                                  _: *mut libc::c_void)
                                 -> libc::c_long>,
    pub get_cipher_by_char: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_uchar)
                                       -> *const SSL_CIPHER>,
    pub put_cipher_by_char: Option<unsafe extern "C" fn(_: *const SSL_CIPHER,
                                                        _: *mut libc::c_uchar)
                                       -> libc::c_int>,
    pub ssl_pending: Option<unsafe extern "C" fn(_: *const SSL)
                                -> libc::c_int>,
    pub num_ciphers: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub get_cipher: Option<unsafe extern "C" fn(_: libc::c_uint)
                               -> *const SSL_CIPHER>,
    pub get_ssl_method: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *const ssl_method_st>,
    pub get_timeout: Option<unsafe extern "C" fn() -> libc::c_long>,
    pub ssl3_enc: *mut ssl3_enc_method,
    pub ssl_version: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub ssl_callback_ctrl: Option<unsafe extern "C" fn(_: *mut SSL,
                                                       _: libc::c_int,
                                                       _:
                                                           Option<unsafe extern "C" fn()
                                                                      -> ()>)
                                      -> libc::c_long>,
    pub ssl_ctx_callback_ctrl: Option<unsafe extern "C" fn(_: *mut SSL_CTX,
                                                           _: libc::c_int,
                                                           _:
                                                               Option<unsafe extern "C" fn()
                                                                          ->
                                                                              ()>)
                                          -> libc::c_long>,
}
pub type tls_session_secret_cb_fn
    =
    Option<unsafe extern "C" fn(_: *mut SSL, _: *mut libc::c_void,
                                _: *mut libc::c_int,
                                _: *mut stack_st_SSL_CIPHER,
                                _: *mut *mut SSL_CIPHER, _: *mut libc::c_void)
               -> libc::c_int>;
pub type tls_session_ticket_ext_cb_fn
    =
    Option<unsafe extern "C" fn(_: *mut SSL, _: *const libc::c_uchar,
                                _: libc::c_int, _: *mut libc::c_void)
               -> libc::c_int>;
pub type TLS_SESSION_TICKET_EXT = tls_session_ticket_ext_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct tls_session_ticket_ext_st {
    pub length: libc::c_ushort,
    pub data: *mut libc::c_void,
}
pub type X509_EXTENSIONS = stack_st_X509_EXTENSION;
pub type COMP_CTX = comp_ctx_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct comp_ctx_st {
    pub meth: *mut COMP_METHOD,
    pub compress_in: libc::c_ulong,
    pub compress_out: libc::c_ulong,
    pub expand_in: libc::c_ulong,
    pub expand_out: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type COMP_METHOD = comp_method_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct comp_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(_: *mut COMP_CTX) -> libc::c_int>,
    pub finish: Option<unsafe extern "C" fn(_: *mut COMP_CTX) -> ()>,
    pub compress: Option<unsafe extern "C" fn(_: *mut COMP_CTX,
                                              _: *mut libc::c_uchar,
                                              _: libc::c_uint,
                                              _: *mut libc::c_uchar,
                                              _: libc::c_uint)
                             -> libc::c_int>,
    pub expand: Option<unsafe extern "C" fn(_: *mut COMP_CTX,
                                            _: *mut libc::c_uchar,
                                            _: libc::c_uint,
                                            _: *mut libc::c_uchar,
                                            _: libc::c_uint) -> libc::c_int>,
    pub ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
    pub callback_ctrl: Option<unsafe extern "C" fn() -> libc::c_long>,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_state_st {
    pub send_cookie: libc::c_uint,
    pub cookie: [libc::c_uchar; 256],
    pub rcvd_cookie: [libc::c_uchar; 256],
    pub cookie_len: libc::c_uint,
    pub r_epoch: libc::c_ushort,
    pub w_epoch: libc::c_ushort,
    pub bitmap: DTLS1_BITMAP,
    pub next_bitmap: DTLS1_BITMAP,
    pub handshake_write_seq: libc::c_ushort,
    pub next_handshake_write_seq: libc::c_ushort,
    pub handshake_read_seq: libc::c_ushort,
    pub last_write_sequence: [libc::c_uchar; 8],
    pub unprocessed_rcds: record_pqueue,
    pub processed_rcds: record_pqueue,
    pub buffered_messages: pqueue,
    pub sent_messages: pqueue,
    pub buffered_app_data: record_pqueue,
    pub listen: libc::c_uint,
    pub link_mtu: libc::c_uint,
    pub mtu: libc::c_uint,
    pub w_msg_hdr: hm_header_st,
    pub r_msg_hdr: hm_header_st,
    pub timeout: dtls1_timeout_st,
    pub next_timeout: timeval,
    pub timeout_duration: libc::c_ushort,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 12],
    pub handshake_fragment_len: libc::c_uint,
    pub retransmitting: libc::c_uint,
    pub change_cipher_spec_ok: libc::c_uint,
}
/* ssl/dtls1.h */
/*
 * DTLS implementation written by Nagendra Modadugu
 * (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
 */
/* ====================================================================
 * Copyright (c) 1999-2005 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.OpenSSL.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@OpenSSL.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.OpenSSL.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
/* Special value for method supporting multiple versions */
/* lengths of messages */
/* Max MTU overhead we know about so far is 40 for IPv6 + 8 for UDP */
/* track 32 packets on 32-bit systems and 64
                                 * - on 64-bit systems */
/* max record number seen so far, 64-bit
                                   * value in big-endian encoding */
/* cryptographic state */
/* used for mac generation */
/* compression */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_timeout_st {
    pub read_timeouts: libc::c_uint,
    pub write_timeouts: libc::c_uint,
    pub num_alerts: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct hm_header_st {
    pub type_0: libc::c_uchar,
    pub msg_len: libc::c_ulong,
    pub seq: libc::c_ushort,
    pub frag_off: libc::c_ulong,
    pub frag_len: libc::c_ulong,
    pub is_ccs: libc::c_uint,
    pub saved_retransmit_state: dtls1_retransmit_state,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_retransmit_state {
    pub enc_write_ctx: *mut EVP_CIPHER_CTX,
    pub write_hash: *mut EVP_MD_CTX,
    pub compress: *mut COMP_CTX,
    pub session: *mut SSL_SESSION,
    pub epoch: libc::c_ushort,
}
pub type record_pqueue = record_pqueue_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct record_pqueue_st {
    pub epoch: libc::c_ushort,
    pub q: pqueue,
}
/* crypto/pqueue/pqueue.h */
/*
 * DTLS implementation written by Nagendra Modadugu
 * (nagendra@cs.stanford.edu) for the OpenSSL project 2005.
 */
/* ====================================================================
 * Copyright (c) 1999-2005 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.OpenSSL.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@OpenSSL.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.OpenSSL.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
pub type pqueue = *mut _pqueue;
pub type DTLS1_BITMAP = dtls1_bitmap_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct dtls1_bitmap_st {
    pub map: libc::c_ulong,
    pub max_seq_num: [libc::c_uchar; 8],
}
/*
 * SSL3_CT_NUMBER is used to size arrays and it must be large enough to
 * contain all of the cert types defined either for SSLv3 and TLSv1.
 */
/*
 * Set when the handshake is ready to process peer's ChangeCipherSpec message.
 * Cleared after the message has been processed.
 */
/* SSL3_FLAGS_SGC_RESTART_DONE is no longer used */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl3_state_st {
    pub flags: libc::c_long,
    pub delay_buf_pop_ret: libc::c_int,
    pub read_sequence: [libc::c_uchar; 8],
    pub read_mac_secret_size: libc::c_int,
    pub read_mac_secret: [libc::c_uchar; 64],
    pub write_sequence: [libc::c_uchar; 8],
    pub write_mac_secret_size: libc::c_int,
    pub write_mac_secret: [libc::c_uchar; 64],
    pub server_random: [libc::c_uchar; 32],
    pub client_random: [libc::c_uchar; 32],
    pub need_empty_fragments: libc::c_int,
    pub empty_fragment_done: libc::c_int,
    pub init_extra: libc::c_int,
    pub rbuf: SSL3_BUFFER,
    pub wbuf: SSL3_BUFFER,
    pub rrec: SSL3_RECORD,
    pub wrec: SSL3_RECORD,
    pub alert_fragment: [libc::c_uchar; 2],
    pub alert_fragment_len: libc::c_uint,
    pub handshake_fragment: [libc::c_uchar; 4],
    pub handshake_fragment_len: libc::c_uint,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_type: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub handshake_buffer: *mut BIO,
    pub handshake_dgst: *mut *mut EVP_MD_CTX,
    pub change_cipher_spec: libc::c_int,
    pub warn_alert: libc::c_int,
    pub fatal_alert: libc::c_int,
    pub alert_dispatch: libc::c_int,
    pub send_alert: [libc::c_uchar; 2],
    pub renegotiate: libc::c_int,
    pub total_renegotiations: libc::c_int,
    pub num_renegotiations: libc::c_int,
    pub in_read_app_data: libc::c_int,
    pub client_opaque_prf_input: *mut libc::c_void,
    pub client_opaque_prf_input_len: size_t,
    pub server_opaque_prf_input: *mut libc::c_void,
    pub server_opaque_prf_input_len: size_t,
    pub tmp: C2RustUnnamed_9,
    pub previous_client_finished: [libc::c_uchar; 64],
    pub previous_client_finished_len: libc::c_uchar,
    pub previous_server_finished: [libc::c_uchar; 64],
    pub previous_server_finished_len: libc::c_uchar,
    pub send_connection_binding: libc::c_int,
    pub next_proto_neg_seen: libc::c_int,
    pub is_probably_safari: libc::c_char,
    pub alpn_selected: *mut libc::c_uchar,
    pub alpn_selected_len: libc::c_uint,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub cert_verify_md: [libc::c_uchar; 128],
    pub finish_md: [libc::c_uchar; 128],
    pub finish_md_len: libc::c_int,
    pub peer_finish_md: [libc::c_uchar; 128],
    pub peer_finish_md_len: libc::c_int,
    pub message_size: libc::c_ulong,
    pub message_type: libc::c_int,
    pub new_cipher: *const SSL_CIPHER,
    pub dh: *mut DH,
    pub ecdh: *mut EC_KEY,
    pub next_state: libc::c_int,
    pub reuse_message: libc::c_int,
    pub cert_req: libc::c_int,
    pub ctype_num: libc::c_int,
    pub ctype: [libc::c_char; 9],
    pub ca_names: *mut stack_st_X509_NAME,
    pub use_rsa_tmp: libc::c_int,
    pub key_block_length: libc::c_int,
    pub key_block: *mut libc::c_uchar,
    pub new_sym_enc: *const EVP_CIPHER,
    pub new_hash: *const EVP_MD,
    pub new_mac_pkey_type: libc::c_int,
    pub new_mac_secret_size: libc::c_int,
    pub new_compression: *const SSL_COMP,
    pub cert_request: libc::c_int,
}
pub type SSL_COMP = ssl_comp_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl_comp_st {
    pub id: libc::c_int,
    pub name: *const libc::c_char,
    pub method: *mut COMP_METHOD,
}
/* crypto/ec/ec.h */
/*
 * Originally written by Bodo Moeller for the OpenSSL project.
 */
/* *
 * \file crypto/ec/ec.h Include file for the OpenSSL EC functions
 * \author Originally written by Bodo Moeller for the OpenSSL project
 */
/* ====================================================================
 * Copyright (c) 1998-2019 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
/* ====================================================================
 * Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
 *
 * Portions of the attached software ("Contribution") are developed by
 * SUN MICROSYSTEMS, INC., and are contributed to the OpenSSL project.
 *
 * The Contribution is licensed pursuant to the OpenSSL open source
 * license provided above.
 *
 * The elliptic curve binary polynomial software is originally written by
 * Sheueling Chang Shantz and Douglas Stebila of Sun Microsystems Laboratories.
 *
 */
/* * Enum for the point conversion form as defined in X9.62 (ECDSA)
 *  for the encoding of a elliptic curve point (x,y) */
/* * the point is encoded as z||x, where the octet z specifies
         *  which solution of the quadratic equation y is  */
/* * the point is encoded as z||x||y, where z is the octet 0x04  */
/* * the point is encoded as z||x||y, where the octet z specifies
         *  which solution of the quadratic equation y is  */
/*-
     EC_METHOD *meth;
     -- field definition
     -- curve coefficients
     -- optional generator with associated information (order, cofactor)
     -- optional extra data (precomputed table for fast computation of multiples of generator)
     -- ASN1 stuff
    */
/* *******************************************************************/
/*               EC_METHODs for curves over GF(p)                   */
/* *******************************************************************/
/* * Returns the basic GFp ec methods which provides the basis for the
 *  optimized methods.
 *  \return  EC_METHOD object
 */
/* * Returns GFp methods using montgomery multiplication.
 *  \return  EC_METHOD object
 */
/* * Returns GFp methods using optimized methods for NIST recommended curves
 *  \return  EC_METHOD object
 */
/* * Returns 64-bit optimized methods for nistp224
 *  \return  EC_METHOD object
 */
/* * Returns 64-bit optimized methods for nistp256
 *  \return  EC_METHOD object
 */
/* * Returns 64-bit optimized methods for nistp521
 *  \return  EC_METHOD object
 */
/* *******************************************************************/
/*           EC_METHOD for curves over GF(2^m)                      */
/* *******************************************************************/
/* * Returns the basic GF2m ec method
 *  \return  EC_METHOD object
 */
/* *******************************************************************/
/*                   EC_GROUP functions                             */
/* *******************************************************************/
/* * Creates a new EC_GROUP object
 *  \param   meth  EC_METHOD to use
 *  \return  newly created EC_GROUP object or NULL in case of an error.
 */
/* * Frees a EC_GROUP object
 *  \param  group  EC_GROUP object to be freed.
 */
/* * Clears and frees a EC_GROUP object
 *  \param  group  EC_GROUP object to be cleared and freed.
 */
/* * Copies EC_GROUP objects. Note: both EC_GROUPs must use the same EC_METHOD.
 *  \param  dst  destination EC_GROUP object
 *  \param  src  source EC_GROUP object
 *  \return 1 on success and 0 if an error occurred.
 */
/* * Creates a new EC_GROUP object and copies the copies the content
 *  form src to the newly created EC_KEY object
 *  \param  src  source EC_GROUP object
 *  \return newly created EC_GROUP object or NULL in case of an error.
 */
/* * Returns the EC_METHOD of the EC_GROUP object.
 *  \param  group  EC_GROUP object
 *  \return EC_METHOD used in this EC_GROUP object.
 */
/* * Returns the field type of the EC_METHOD.
 *  \param  meth  EC_METHOD object
 *  \return NID of the underlying field type OID.
 */
/* * Sets the generator and it's order/cofactor of a EC_GROUP object.
 *  \param  group      EC_GROUP object
 *  \param  generator  EC_POINT object with the generator.
 *  \param  order      the order of the group generated by the generator.
 *  \param  cofactor   the index of the sub-group generated by the generator
 *                     in the group of all points on the elliptic curve.
 *  \return 1 on success and 0 if an error occured
 */
/* * Returns the generator of a EC_GROUP object.
 *  \param  group  EC_GROUP object
 *  \return the currently used generator (possibly NULL).
 */
/* * Returns the montgomery data for order(Generator)
 *  \param  group  EC_GROUP object
 *  \return the currently used generator (possibly NULL).
*/
/* * Gets the order of a EC_GROUP
 *  \param  group  EC_GROUP object
 *  \param  order  BIGNUM to which the order is copied
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Gets the cofactor of a EC_GROUP
 *  \param  group     EC_GROUP object
 *  \param  cofactor  BIGNUM to which the cofactor is copied
 *  \param  ctx       BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Sets the name of a EC_GROUP object
 *  \param  group  EC_GROUP object
 *  \param  nid    NID of the curve name OID
 */
/* * Returns the curve name of a EC_GROUP object
 *  \param  group  EC_GROUP object
 *  \return NID of the curve name OID or 0 if not set.
 */
/* * Sets the parameter of a ec over GFp defined by y^2 = x^3 + a*x + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM with the prime number
 *  \param  a      BIGNUM with parameter a of the equation
 *  \param  b      BIGNUM with parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Gets the parameter of the ec over GFp defined by y^2 = x^3 + a*x + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM for the prime number
 *  \param  a      BIGNUM for parameter a of the equation
 *  \param  b      BIGNUM for parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Sets the parameter of a ec over GF2m defined by y^2 + x*y = x^3 + a*x^2 + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM with the polynomial defining the underlying field
 *  \param  a      BIGNUM with parameter a of the equation
 *  \param  b      BIGNUM with parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Gets the parameter of the ec over GF2m defined by y^2 + x*y = x^3 + a*x^2 + b
 *  \param  group  EC_GROUP object
 *  \param  p      BIGNUM for the polynomial defining the underlying field
 *  \param  a      BIGNUM for parameter a of the equation
 *  \param  b      BIGNUM for parameter b of the equation
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Returns the number of bits needed to represent a field element
 *  \param  group  EC_GROUP object
 *  \return number of bits needed to represent a field element
 */
/* * Checks whether the parameter in the EC_GROUP define a valid ec group
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if group is a valid ec group and 0 otherwise
 */
/* * Checks whether the discriminant of the elliptic curve is zero or not
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if the discriminant is not zero and 0 otherwise
 */
/* * Compares two EC_GROUP objects
 *  \param  a    first EC_GROUP object
 *  \param  b    second EC_GROUP object
 *  \param  ctx  BN_CTX object (optional)
 *  \return 0 if both groups are equal and 1 otherwise
 */
/*
 * EC_GROUP_new_GF*() calls EC_GROUP_new() and EC_GROUP_set_GF*() after
 * choosing an appropriate EC_METHOD
 */
/* * Creates a new EC_GROUP object with the specified parameters defined
 *  over GFp (defined by the equation y^2 = x^3 + a*x + b)
 *  \param  p    BIGNUM with the prime number
 *  \param  a    BIGNUM with the parameter a of the equation
 *  \param  b    BIGNUM with the parameter b of the equation
 *  \param  ctx  BN_CTX object (optional)
 *  \return newly created EC_GROUP object with the specified parameters
 */
/* * Creates a new EC_GROUP object with the specified parameters defined
 *  over GF2m (defined by the equation y^2 + x*y = x^3 + a*x^2 + b)
 *  \param  p    BIGNUM with the polynomial defining the underlying field
 *  \param  a    BIGNUM with the parameter a of the equation
 *  \param  b    BIGNUM with the parameter b of the equation
 *  \param  ctx  BN_CTX object (optional)
 *  \return newly created EC_GROUP object with the specified parameters
 */
/* * Creates a EC_GROUP object with a curve specified by a NID
 *  \param  nid  NID of the OID of the curve name
 *  \return newly created EC_GROUP object with specified curve or NULL
 *          if an error occurred
 */
/* *******************************************************************/
/*               handling of internal curves                        */
/* *******************************************************************/
/*
 * EC_builtin_curves(EC_builtin_curve *r, size_t size) returns number of all
 * available curves or zero if a error occurred. In case r ist not zero
 * nitems EC_builtin_curve structures are filled with the data of the first
 * nitems internal groups
 */
/* *******************************************************************/
/*                    EC_POINT functions                            */
/* *******************************************************************/
/* * Creates a new EC_POINT object for the specified EC_GROUP
 *  \param  group  EC_GROUP the underlying EC_GROUP object
 *  \return newly created EC_POINT object or NULL if an error occurred
 */
/* * Frees a EC_POINT object
 *  \param  point  EC_POINT object to be freed
 */
/* * Clears and frees a EC_POINT object
 *  \param  point  EC_POINT object to be cleared and freed
 */
/* * Copies EC_POINT object
 *  \param  dst  destination EC_POINT object
 *  \param  src  source EC_POINT object
 *  \return 1 on success and 0 if an error occured
 */
/* * Creates a new EC_POINT object and copies the content of the supplied
 *  EC_POINT
 *  \param  src    source EC_POINT object
 *  \param  group  underlying the EC_GROUP object
 *  \return newly created EC_POINT object or NULL if an error occurred
 */
/* * Returns the EC_METHOD used in EC_POINT object
 *  \param  point  EC_POINT object
 *  \return the EC_METHOD used
 */
/* * Sets a point to infinity (neutral element)
 *  \param  group  underlying EC_GROUP object
 *  \param  point  EC_POINT to set to infinity
 *  \return 1 on success and 0 if an error occured
 */
/* * Sets the jacobian projective coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  z      BIGNUM with the z-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Gets the jacobian projective coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  z      BIGNUM for the z-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Sets the affine coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Gets the affine coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Sets the x9.62 compressed coordinates of a EC_POINT over GFp
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with x-coordinate
 *  \param  y_bit  integer with the y-Bit (either 0 or 1)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Sets the affine coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with the x-coordinate
 *  \param  y      BIGNUM with the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Gets the affine coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM for the x-coordinate
 *  \param  y      BIGNUM for the y-coordinate
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Sets the x9.62 compressed coordinates of a EC_POINT over GF2m
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  x      BIGNUM with x-coordinate
 *  \param  y_bit  integer with the y-Bit (either 0 or 1)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Encodes a EC_POINT object to a octet string
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  form   point conversion form
 *  \param  buf    memory buffer for the result. If NULL the function returns
 *                 required buffer size.
 *  \param  len    length of the memory buffer
 *  \param  ctx    BN_CTX object (optional)
 *  \return the length of the encoded octet string or 0 if an error occurred
 */
/* * Decodes a EC_POINT from a octet string
 *  \param  group  underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \param  buf    memory buffer with the encoded ec point
 *  \param  len    length of the encoded ec point
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* other interfaces to point2oct/oct2point: */
/* *******************************************************************/
/*         functions for doing EC_POINT arithmetic                  */
/* *******************************************************************/
/* * Computes the sum of two EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result (r = a + b)
 *  \param  a      EC_POINT object with the first summand
 *  \param  b      EC_POINT object with the second summand
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Computes the double of a EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result (r = 2 * a)
 *  \param  a      EC_POINT object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Computes the inverse of a EC_POINT
 *  \param  group  underlying EC_GROUP object
 *  \param  a      EC_POINT object to be inverted (it's used for the result as well)
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Checks whether the point is the neutral element of the group
 *  \param  group  the underlying EC_GROUP object
 *  \param  p      EC_POINT object
 *  \return 1 if the point is the neutral element and 0 otherwise
 */
/* * Checks whether the point is on the curve
 *  \param  group  underlying EC_GROUP object
 *  \param  point  EC_POINT object to check
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 if point if on the curve and 0 otherwise
 */
/* * Compares two EC_POINTs
 *  \param  group  underlying EC_GROUP object
 *  \param  a      first EC_POINT object
 *  \param  b      second EC_POINT object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 0 if both points are equal and a value != 0 otherwise
 */
/* * Computes r = generator * n sum_{i=0}^{num-1} p[i] * m[i]
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result
 *  \param  n      BIGNUM with the multiplier for the group generator (optional)
 *  \param  num    number futher summands
 *  \param  p      array of size num of EC_POINT objects
 *  \param  m      array of size num of BIGNUM objects
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Computes r = generator * n + q * m
 *  \param  group  underlying EC_GROUP object
 *  \param  r      EC_POINT object for the result
 *  \param  n      BIGNUM with the multiplier for the group generator (optional)
 *  \param  q      EC_POINT object with the first factor of the second summand
 *  \param  m      BIGNUM with the second factor of the second summand
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Stores multiples of generator for faster point multiplication
 *  \param  group  EC_GROUP object
 *  \param  ctx    BN_CTX object (optional)
 *  \return 1 on success and 0 if an error occured
 */
/* * Reports whether a precomputation has been done
 *  \param  group  EC_GROUP object
 *  \return 1 if a pre-computation has been done and 0 otherwise
 */
/* *******************************************************************/
/*                       ASN1 stuff                                 */
/* *******************************************************************/
/*
 * EC_GROUP_get_basis_type() returns the NID of the basis type used to
 * represent the field elements
 */
/* *******************************************************************/
/*                      EC_KEY functions                            */
/* *******************************************************************/
pub type EC_KEY = ec_key_st;
/* crypto/bio/bio.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* These are the 'types' of BIOs */
/* passive filter */
/* filter */
/* filter */
/* filter */
/* socket - connect */
/* socket for accept */
/* client proxy BIO */
/* server proxy BIO */
/* server proxy BIO */
/* BER -> bin filter */
/* (half a) BIO pair */
/* filter */
/* filter */
/* filter */
/* socket, fd, connect or accept */
/*
 * BIO_FILENAME_READ|BIO_CLOSE to open or close on free.
 * BIO_set_fp(in,stdin,BIO_NOCLOSE);
 */
/*
 * These are used in the following macros and are passed to BIO_ctrl()
 */
/* opt - rewind/zero etc */
/* opt - are we at the eof */
/* opt - extra tit-bits */
/* man - set the 'IO' type */
/* man - get the 'IO' type */
/* opt - internal, used to signify change */
/* opt - internal, used to signify change */
/* man - set the 'close' on free */
/* man - set the 'close' on free */
/* opt - is their more data buffered */
/* opt - 'flush' buffered output */
/* man - extra stuff for 'duped' BIO */
/* opt - number of bytes still to write */
/* callback is int cb(BIO *bio,state,ret); */
/* opt - set callback function */
/* opt - set callback function */
/* BIO_s_file special */
/* dgram BIO stuff */
/* BIO dgram special */
/* allow for an externally connected
                                         * socket to be passed in */
/* setsockopt, essentially */
/* getsockopt, essentially */
/* setsockopt, essentially */
/* getsockopt, essentially */
/* flag whether the last */
/* I/O operation tiemd out */
/* #ifdef IP_MTU_DISCOVER */
/* set DF bit on egress packets */
/* #endif */
/* as kernel for current MTU */
/* get cached value for MTU */
/* set cached value for MTU.
                                              * want to use this if asking
                                              * the kernel fails */
/* check whether the MTU was
                                              * exceed in the previous write
                                              * operation */
/* Destination for the data */
/* Next DTLS handshake timeout
                                              * to adjust socket timeouts */
/* modifiers */
/*
 * "UPLINK" flag denotes file descriptors provided by application. It
 * defaults to 0, as most platforms don't require UPLINK interface.
 */
/* Used in BIO_gethostbyname() */
/* Mostly used in the SSL BIO */
/*-
 * Not used anymore
 * #define BIO_FLAGS_PROTOCOL_DELAYED_READ 0x10
 * #define BIO_FLAGS_PROTOCOL_DELAYED_WRITE 0x20
 * #define BIO_FLAGS_PROTOCOL_STARTUP   0x40
 */
/*
 * This is used with memory BIOs: it means we shouldn't free up or change the
 * data in any way.
 */
pub type BIO = bio_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bio_st {
    pub method: *mut BIO_METHOD,
    pub callback: Option<unsafe extern "C" fn(_: *mut bio_st, _: libc::c_int,
                                              _: *const libc::c_char,
                                              _: libc::c_int, _: libc::c_long,
                                              _: libc::c_long)
                             -> libc::c_long>,
    pub cb_arg: *mut libc::c_char,
    pub init: libc::c_int,
    pub shutdown: libc::c_int,
    pub flags: libc::c_int,
    pub retry_reason: libc::c_int,
    pub num: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub next_bio: *mut bio_st,
    pub prev_bio: *mut bio_st,
    pub references: libc::c_int,
    pub num_read: libc::c_ulong,
    pub num_write: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
    /*
 * This stuff is basically class callback functions The current classes are
 * SSL_CTX, SSL, SSL_SESSION, and a few more
 */
    /* Arbitary long */
    /* Arbitary void * */
    /*
 * Per class, we have a STACK of CRYPTO_EX_DATA_FUNCS for each CRYPTO_EX_DATA
 * entry.
 */
}
pub type BIO_METHOD = bio_method_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bio_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub bwrite: Option<unsafe extern "C" fn(_: *mut BIO,
                                            _: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
    pub bread: Option<unsafe extern "C" fn(_: *mut BIO, _: *mut libc::c_char,
                                           _: libc::c_int) -> libc::c_int>,
    pub bputs: Option<unsafe extern "C" fn(_: *mut BIO,
                                           _: *const libc::c_char)
                          -> libc::c_int>,
    pub bgets: Option<unsafe extern "C" fn(_: *mut BIO, _: *mut libc::c_char,
                                           _: libc::c_int) -> libc::c_int>,
    pub ctrl: Option<unsafe extern "C" fn(_: *mut BIO, _: libc::c_int,
                                          _: libc::c_long,
                                          _: *mut libc::c_void)
                         -> libc::c_long>,
    pub create: Option<unsafe extern "C" fn(_: *mut BIO) -> libc::c_int>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut BIO) -> libc::c_int>,
    pub callback_ctrl: Option<unsafe extern "C" fn(_: *mut BIO,
                                                   _: libc::c_int,
                                                   _:
                                                       Option<unsafe extern "C" fn(_:
                                                                                       *mut bio_st,
                                                                                   _:
                                                                                       libc::c_int,
                                                                                   _:
                                                                                       *const libc::c_char,
                                                                                   _:
                                                                                       libc::c_int,
                                                                                   _:
                                                                                       libc::c_long,
                                                                                   _:
                                                                                       libc::c_long)
                                                                  -> ()>)
                                  -> libc::c_long>,
}
pub type bio_info_cb
    =
    unsafe extern "C" fn(_: *mut bio_st, _: libc::c_int,
                         _: *const libc::c_char, _: libc::c_int,
                         _: libc::c_long, _: libc::c_long) -> ();
/* ssl/ssl3.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* ====================================================================
 * Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * 3. All advertising materials mentioning features or use of this
 *    software must display the following acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit. (http://www.openssl.org/)"
 *
 * 4. The names "OpenSSL Toolkit" and "OpenSSL Project" must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission. For written permission, please contact
 *    openssl-core@openssl.org.
 *
 * 5. Products derived from this software may not be called "OpenSSL"
 *    nor may "OpenSSL" appear in their names without prior written
 *    permission of the OpenSSL Project.
 *
 * 6. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by the OpenSSL Project
 *    for use in the OpenSSL Toolkit (http://www.openssl.org/)"
 *
 * THIS SOFTWARE IS PROVIDED BY THE OpenSSL PROJECT ``AS IS'' AND ANY
 * EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE OpenSSL PROJECT OR
 * ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 * ====================================================================
 *
 * This product includes cryptographic software written by Eric Young
 * (eay@cryptsoft.com).  This product includes software written by Tim
 * Hudson (tjh@cryptsoft.com).
 *
 */
/* ====================================================================
 * Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
 * ECC cipher suite support in OpenSSL originally developed by
 * SUN MICROSYSTEMS, INC., and contributed to the OpenSSL project.
 */
/*
 * Signalling cipher suite value from RFC 5746
 * (TLS_EMPTY_RENEGOTIATION_INFO_SCSV)
 */
/*
 * Signalling cipher suite value from draft-ietf-tls-downgrade-scsv-00
 * (TLS_FALLBACK_SCSV)
 */
/*
 * VRS Additional Kerberos5 entries
 */
/*
 * This next block of six "EDH" labels is for backward compatibility with
 * older versions of OpenSSL.  New code should use the six "DHE" labels above
 * instead:
 */
/*
  * Some will argue that this increases memory footprint, but it's not
  * actually true. Point is that malloc has to return at least 64-bit aligned
  * pointers, meaning that allocating 5 bytes wastes 3 bytes in either case.
  * Suggested pre-gaping simply moves these wasted bytes from the end of
  * allocated region to its front, but makes data payload aligned, which
  * improves performance:-)
  */
/*
 * This is the maximum MAC (digest) size used by the SSL library. Currently
 * maximum of 20 is used by SHA1, but we reserve for future extension for
 * 512-bit hashes.
 */
/*
 * Maximum block size used in all ciphersuites. Currently 16 for AES.
 */
/* Maximum plaintext length: defined by SSL/TLS standards */
/* Maximum compression overhead: defined by SSL/TLS standards */
/*
 * The standards give a maximum encryption overhead of 1024 bytes. In
 * practice the value is lower than this. The overhead is the maximum number
 * of padding bytes (256) plus the mac size.
 */
/*
 * OpenSSL currently only uses a padding length of at most one block so the
 * send overhead is smaller.
 */
/* If compression isn't used don't include the compression overhead */
/* Pseudo content types to indicate additional parameters */
/* Pseudo content type for SSL/TLS header info */
/* fatal */
/* fatal */
/* fatal */
/* fatal */
/* fatal */
pub type SSL3_RECORD = ssl3_record_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl3_record_st {
    pub type_0: libc::c_int,
    pub length: libc::c_uint,
    pub off: libc::c_uint,
    pub data: *mut libc::c_uchar,
    pub input: *mut libc::c_uchar,
    pub comp: *mut libc::c_uchar,
    pub epoch: libc::c_ulong,
    pub seq_num: [libc::c_uchar; 8],
}
pub type SSL3_BUFFER = ssl3_buffer_st;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl3_buffer_st {
    pub buf: *mut libc::c_uchar,
    pub len: size_t,
    pub offset: libc::c_int,
    pub left: libc::c_int,
}
/* type of record */
    /*
     * r
     */
/* How many bytes available */
    /*
     * rw
     */
/* read/write offset into 'buf' */
    /*
     * r
     */
/* pointer to the record data */
    /*
     * rw
     */
/* where the decode bytes are */
    /*
     * rw
     */
/* only used with decompression - malloc()ed */
    /*
     * r
     */
/* epoch number, needed by DTLS1 */
    /*
     * r
     */
/* sequence number, needed by DTLS1 */
    /*
     * r
     */
/* at least SSL3_RT_MAX_PACKET_SIZE bytes, see ssl3_setup_buffers() */
/* buffer size */
/* where to 'copy from' */
/* how many bytes left */
/* ssl/ssl2.h */
/* Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
 * All rights reserved.
 *
 * This package is an SSL implementation written
 * by Eric Young (eay@cryptsoft.com).
 * The implementation was written so as to conform with Netscapes SSL.
 *
 * This library is free for commercial and non-commercial use as long as
 * the following conditions are aheared to.  The following conditions
 * apply to all code found in this distribution, be it the RC4, RSA,
 * lhash, DES, etc., code; not just the SSL code.  The SSL documentation
 * included with this distribution is covered by the same copyright terms
 * except that the holder is Tim Hudson (tjh@cryptsoft.com).
 *
 * Copyright remains Eric Young's, and as such any Copyright notices in
 * the code are not to be removed.
 * If this package is used in a product, Eric Young should be given attribution
 * as the author of the parts of the library used.
 * This can be in the form of a textual message at program startup or
 * in documentation (online or textual) provided with the package.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *    "This product includes cryptographic software written by
 *     Eric Young (eay@cryptsoft.com)"
 *    The word 'cryptographic' can be left out if the rouines from the library
 *    being used are not cryptographic related :-).
 * 4. If you include any Windows specific code (or a derivative thereof) from
 *    the apps directory (application code) you must include an acknowledgement:
 *    "This product includes software written by Tim Hudson (tjh@cryptsoft.com)"
 *
 * THIS SOFTWARE IS PROVIDED BY ERIC YOUNG ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * The licence and distribution terms for any publically available version or
 * derivative of this code cannot be changed.  i.e. this code cannot simply be
 * copied and put under another distribution licence
 * [including the GNU Public Licence.]
 */
/* Protocol Version Codes */
/* #define SSL2_CLIENT_VERSION  0x0002 */
/* #define SSL2_SERVER_VERSION  0x0002 */
/* Protocol Message Codes */
/* Error Message Codes */
/* Cipher Kind Values */
/* v3 */
/* v3 */
/* v3 */
/* MS hack */
/* SSLeay */
/* SSLeay */
/* Flags for the SSL_CIPHER.algorithm2 field */
/* Certificate Type Codes */
/* Authentication Type Code */
/* Upper/Lower Bounds */
/* 2^15-1 */
/* 2^14-1 */
/*
 * #define SSL2_CHALLENGE_LENGTH 32
 */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct ssl2_state_st {
    pub three_byte_header: libc::c_int,
    pub clear_text: libc::c_int,
    pub escape: libc::c_int,
    pub ssl2_rollback: libc::c_int,
    pub wnum: libc::c_uint,
    pub wpend_tot: libc::c_int,
    pub wpend_buf: *const libc::c_uchar,
    pub wpend_off: libc::c_int,
    pub wpend_len: libc::c_int,
    pub wpend_ret: libc::c_int,
    pub rbuf_left: libc::c_int,
    pub rbuf_offs: libc::c_int,
    pub rbuf: *mut libc::c_uchar,
    pub wbuf: *mut libc::c_uchar,
    pub write_ptr: *mut libc::c_uchar,
    pub padding: libc::c_uint,
    pub rlength: libc::c_uint,
    pub ract_data_length: libc::c_int,
    pub wlength: libc::c_uint,
    pub wact_data_length: libc::c_int,
    pub ract_data: *mut libc::c_uchar,
    pub wact_data: *mut libc::c_uchar,
    pub mac_data: *mut libc::c_uchar,
    pub read_key: *mut libc::c_uchar,
    pub write_key: *mut libc::c_uchar,
    pub challenge_length: libc::c_uint,
    pub challenge: [libc::c_uchar; 32],
    pub conn_id_length: libc::c_uint,
    pub conn_id: [libc::c_uchar; 16],
    pub key_material_length: libc::c_uint,
    pub key_material: [libc::c_uchar; 48],
    pub read_sequence: libc::c_ulong,
    pub write_sequence: libc::c_ulong,
    pub tmp: C2RustUnnamed_10,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub conn_id_length: libc::c_uint,
    pub cert_type: libc::c_uint,
    pub cert_length: libc::c_uint,
    pub csl: libc::c_uint,
    pub clear: libc::c_uint,
    pub enc: libc::c_uint,
    pub ccl: [libc::c_uchar; 32],
    pub cipher_spec_length: libc::c_uint,
    pub session_id_length: libc::c_uint,
    pub clen: libc::c_uint,
    pub rlen: libc::c_uint,
}
/* zlib.h -- interface of the 'zlib' general purpose compression library
  version 1.2.11, January 15th, 2017

  Copyright (C) 1995-2017 Jean-loup Gailly and Mark Adler

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

  Jean-loup Gailly        Mark Adler
  jloup@gzip.org          madler@alumni.caltech.edu


  The data format used by the zlib library is described by RFCs (Request for
  Comments) 1950 to 1952 in the files http://tools.ietf.org/html/rfc1950
  (zlib format), rfc1951 (deflate format) and rfc1952 (gzip format).
*/
/* !__APPLE__ */
/* !__APPLE__ */
/*
    The 'zlib' compression library provides in-memory compression and
  decompression functions, including integrity checks of the uncompressed data.
  This version of the library supports only one compression method (deflation)
  but other algorithms will be added later and will have the same stream
  interface.

    Compression can be done in a single step if the buffers are large enough,
  or can be done by repeated calls of the compression function.  In the latter
  case, the application must provide more input and/or consume the output
  (providing more output space) before each call.

    The compressed data format used by default by the in-memory functions is
  the zlib format, which is a zlib wrapper documented in RFC 1950, wrapped
  around a deflate stream, which is itself documented in RFC 1951.

    The library also supports reading and writing files in gzip (.gz) format
  with an interface similar to that of stdio using the functions that start
  with "gz".  The gzip format is different from the zlib format.  gzip is a
  gzip wrapper, documented in RFC 1952, wrapped around a deflate stream.

    This library can optionally read and write gzip and raw deflate streams in
  memory as well.

    The zlib format was designed to be compact and fast for use in memory
  and on communications channels.  The gzip format was designed for single-
  file compression on file systems, has a larger header than zlib to maintain
  directory information, and uses a different, slower check method than zlib.

    The library does not install any signal handler.  The decoder checks
  the consistency of the compressed data, so the library should never crash
  even in the case of corrupted input.
*/
pub type z_stream = z_stream_s;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
/* next input byte */
/* number of bytes available at next_in */
/* total number of input bytes read so far */
/* next output byte will go here */
/* remaining free space at next_out */
/* total number of bytes output so far */
/* last error message, NULL if no error */
/* not visible by applications */
/* used to allocate the internal state */
/* used to free the internal state */
/* private data object passed to zalloc and zfree */
/* best guess about the data type: binary or text
                           for deflate, or the decoding state for inflate */
/* Adler-32 or CRC-32 value of the uncompressed data */
/* reserved for future use */
/* zconf.h -- configuration of the zlib compression library
 * Copyright (C) 1995-2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/*
 * If you *really* need a unique prefix for all types and library functions,
 * compile with -DZ_PREFIX. The "standard" zlib should be compiled without it.
 * Even better than compiling with -DZ_PREFIX would be to use configure to set
 * this permanently in zconf.h using "./configure --zprefix".
 */
/* may be set to #if 1 by ./configure */
/*
 * Compile with -DMAXSEG_64K if the alloc function cannot allocate more
 * than 64k bytes at a time (needed on systems with 16-bit int).
 */
/* iSeries (formerly AS/400). */
/* Maximum value for memLevel in deflateInit2 */
/* Maximum value for windowBits in deflateInit2 and inflateInit2.
 * WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
 * created by gzip. (Files created by minigzip can still be extracted by
 * gzip.)
 */
/* 32K LZ77 window */
/* The memory requirements for deflate are (in bytes):
            (1 << (windowBits+2)) +  (1 << (memLevel+9))
 that is: 128K for windowBits=15  +  128K for memLevel = 8  (default values)
 plus a few kilobytes for small objects. For example, if you want to reduce
 the default memory requirements from 256K to 128K, compile with
     make CFLAGS="-O -DMAX_WBITS=14 -DMAX_MEM_LEVEL=7"
 Of course this will generally degrade compression (there's no free lunch).

   The memory requirements for inflate are (in bytes) 1 << windowBits
 that is, 32K for windowBits=15 (default value) plus about 7 kilobytes
 for small objects.
*/
/* Type declarations */
/* function prototypes */
/* function prototypes for stdarg */
/* The following definitions for FAR are needed only for MSDOS mixed
 * model programming (small or medium model with some far allocations).
 * This was tested only with MSC; for other MSDOS compilers you may have
 * to define NO_MEMCPY in zutil.h.  If you don't need the mixed model,
 * just define FAR to be empty.
 */
/* 8 bits */
/* 16 bits or more */
pub type uLong = libc::c_ulong;
/* 32 bits or more */
pub type voidpf = *mut libc::c_void;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
pub type alloc_func
    =
    Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type uInt = libc::c_uint;
pub type Bytef = Byte;
pub type Byte = libc::c_uchar;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct pin_cache {
    pub next: *mut pin_cache,
    pub token: *mut libc::c_char,
    pub pin: *mut libc::c_char,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_text_buf {
    pub data: *mut libc::c_char,
    pub pos: libc::c_int,
    pub buf_len: libc::c_int,
    pub error: libc::c_int,
}
pub type openconnect_unlock_token_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char)
               -> libc::c_int>;
pub type openconnect_lock_token_vfn
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const HOTP_SECRET_PSKC: C2RustUnnamed_11 = 4;
pub const HOTP_SECRET_HEX: C2RustUnnamed_11 = 3;
pub const HOTP_SECRET_RAW: C2RustUnnamed_11 = 2;
pub const HOTP_SECRET_BASE32: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const OATH_ALG_HMAC_SHA512: C2RustUnnamed_12 = 2;
pub const OATH_ALG_HMAC_SHA256: C2RustUnnamed_12 = 1;
pub const OATH_ALG_HMAC_SHA1: C2RustUnnamed_12 = 0;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct http_auth_state {
    pub state: libc::c_int,
    pub challenge: *mut libc::c_char,
    pub c2rust_unnamed: C2RustUnnamed_13,
}
#[derive ( Copy, Clone )]
#[repr ( C )]
pub union C2RustUnnamed_13 {
    pub c2rust_unnamed: C2RustUnnamed_15,
    pub c2rust_unnamed_0: C2RustUnnamed_14,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub ntlm_helper_fd: libc::c_int,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub gss_target_name: gss_name_t,
    pub gss_context: gss_ctx_id_t,
}
/*
 * This file is auto generated.  Please do not edit it.
 */
/* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Determine platform-dependent configuration.
 */
/* __cplusplus */
/*
 * First, include stddef.h to get size_t defined.
 */
/*
 * POSIX says that sys/types.h is where size_t is defined.
 */
/*
 * $Id: gssapi.hin 20876 2008-10-15 21:58:43Z tlyu $
 */
/*
 * First, define the three platform-dependent pointer types.
 */
pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
pub type gss_name_t = *mut gss_name_struct;
/* ******************************************************************************
 * libproxy - A library for proxy configuration
 * Copyright (C) 2006 Nathaniel McCallum <nathaniel@natemccallum.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301 USA
 ******************************************************************************/
pub type pxProxyFactory = pxProxyFactory_;
/*
 * Summary: interfaces for tree manipulation
 * Description: this module describes the structures found in an tree resulting
 *              from an XML or HTML parsing, as well as the API provided for
 *              various processing on that tree
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/*
 * Some of the basic types pointer to structures:
 */
/* xmlIO.h */
/* parser.h */
/* entities.h */
/* *
 * BASE_BUFFER_SIZE:
 *
 * default buffer size 4000.
 */
/* *
 * LIBXML_NAMESPACE_DICT:
 *
 * Defines experimental behaviour:
 * 1) xmlNs gets an additional field @context (a xmlDoc)
 * 2) when creating a tree, xmlNs->href is stored in the dict of xmlDoc.
 */
/* #define LIBXML_NAMESPACE_DICT */
/* *
 * xmlBufferAllocationScheme:
 *
 * A buffer allocation scheme can be defined to either match exactly the
 * need or double it's allocated size each time it is found too small.
 */
/* double each time one need to grow */
/* grow only to the minimal size */
/* immutable buffer */
/* special allocation scheme used for I/O */
/* exact up to a threshold, and doubleit thereafter */
/* limit the upper size of the buffer */
/* *
 * xmlBuffer:
 *
 * A buffer structure, this old construct is limited to 2GB and
 * is being deprecated, use API with xmlBuf instead
 */
/* The buffer content UTF8 */
/* The buffer size used */
/* The buffer size */
/* The realloc method */
/* in IO mode we may have a different base */
/* *
 * xmlBuf:
 *
 * A buffer structure, new one, the actual structure internals are not public
 */
/* *
 * xmlBufPtr:
 *
 * A pointer to a buffer structure, the actual structure internals are not
 * public
 */
/*
 * A few public routines for xmlBuf. As those are expected to be used
 * mostly internally the bulk of the routines are internal in buf.h
 */
/*
 * LIBXML2_NEW_BUFFER:
 *
 * Macro used to express that the API use the new buffers for
 * xmlParserInputBuffer and xmlOutputBuffer. The change was
 * introduced in 2.9.0.
 */
/* *
 * XML_XML_NAMESPACE:
 *
 * This is the namespace for the special xml: prefix predefined in the
 * XML Namespace specification.
 */
/* *
 * XML_XML_ID:
 *
 * This is the name for the special xml:id attribute
 */
/*
 * The different element types carried by an XML tree.
 *
 * NOTE: This is synchronized with DOM Level1 values
 *       See http://www.w3.org/TR/REC-DOM-Level-1/
 *
 * Actually this had diverged a bit, and now XML_DOCUMENT_TYPE_NODE should
 * be deprecated to use an XML_DTD_NODE.
 */
/* *
 * xmlNotation:
 *
 * A DTD Notation definition.
 */
/* Notation name */
/* Public identifier, if any */
/* System identifier, if any */
/* *
 * xmlAttributeType:
 *
 * A DTD Attribute type definition.
 */
/* *
 * xmlAttributeDefault:
 *
 * A DTD Attribute default definition.
 */
/* *
 * xmlEnumeration:
 *
 * List structure used when there is an enumeration in DTDs.
 */
/* next one */
/* Enumeration name */
/* *
 * xmlAttribute:
 *
 * An Attribute declaration in a DTD.
 */
/* application data */
/* XML_ATTRIBUTE_DECL, must be second ! */
/* Attribute name */
/* NULL */
/* NULL */
/* -> DTD */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* next in hash table */
/* The attribute type */
/* the default */
/* or the default value */
/* or the enumeration tree if any */
/* the namespace prefix if any */
/* Element holding the attribute */
/* *
 * xmlElementContentType:
 *
 * Possible definitions of element content types.
 */
/* *
 * xmlElementContentOccur:
 *
 * Possible definitions of element content occurrences.
 */
/* *
 * xmlElementContent:
 *
 * An XML Element content as stored after parsing an element definition
 * in a DTD.
 */
/* PCDATA, ELEMENT, SEQ or OR */
/* ONCE, OPT, MULT or PLUS */
/* Element name */
/* first child */
/* second child */
/* parent */
/* Namespace prefix */
/* *
 * xmlElementTypeVal:
 *
 * The different possibilities for an element content type.
 */
/* *
 * xmlElement:
 *
 * An XML Element declaration from a DTD.
 */
/* application data */
/* XML_ELEMENT_DECL, must be second ! */
/* Element name */
/* NULL */
/* NULL */
/* -> DTD */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* The type */
/* the allowed element content */
/* List of the declared attributes */
/* the namespace prefix if any */
/* the validating regexp */
/* *
 * XML_LOCAL_NAMESPACE:
 *
 * A namespace declaration node.
 */
/* *
 * xmlNs:
 *
 * An XML namespace.
 * Note that prefix == NULL is valid, it defines the default namespace
 * within the subtree (until overridden).
 *
 * xmlNsType is unified with xmlElementType.
 */
/* next Ns link for this node  */
/* global or local */
/* URL for the namespace */
/* prefix for the namespace */
/* application data */
/* normally an xmlDoc */
/* *
 * xmlDtd:
 *
 * An XML DTD, as defined by <!DOCTYPE ... There is actually one for
 * the internal subset and for the external subset.
 */
/* application data */
/* XML_DTD_NODE, must be second ! */
/* Name of the DTD */
/* the value of the property link */
/* last child link */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* End of common part */
/* Hash table for notations if any */
/* Hash table for elements if any */
/* Hash table for attributes if any */
/* Hash table for entities if any */
/* External identifier for PUBLIC DTD */
/* URI for a SYSTEM or PUBLIC DTD */
/* Hash table for param entities if any */
/* *
 * xmlAttr:
 *
 * An attribute on an XML node.
 */
/* application data */
/* XML_ATTRIBUTE_NODE, must be second ! */
/* the name of the property */
/* the value of the property */
/* NULL */
/* child->parent link */
/* next sibling link  */
/* previous sibling link  */
/* the containing document */
/* pointer to the associated namespace */
/* the attribute type if validating */
/* for type/PSVI informations */
/* *
 * xmlID:
 *
 * An XML ID instance.
 */
/* next ID */
/* The ID name */
/* The attribute holding it */
/* The attribute if attr is not available */
/* The line number if attr is not available */
/* The document holding the ID */
/* *
 * xmlRef:
 *
 * An XML IDREF instance.
 */
/* next Ref */
/* The Ref name */
/* The attribute holding it */
/* The attribute if attr is not available */
/* The line number if attr is not available */
/* *
 * xmlNode:
 *
 * A node in an XML tree.
 */
pub type xmlNode = _xmlNode;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut libc::c_void,
    pub line: libc::c_ushort,
    pub extra: libc::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut libc::c_void,
    pub context: *mut _xmlDoc,
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *mut libc::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: libc::c_int,
    pub standalone: libc::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut libc::c_void,
    pub refs: *mut libc::c_void,
    pub URL: *const xmlChar,
    pub charset: libc::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut libc::c_void,
    pub parseFlags: libc::c_int,
    pub properties: libc::c_int,
}
/*
 * Summary: set of routines to process strings
 * Description: type and interfaces needed for the internal string handling
 *              of the library, especially UTF8 processing.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlChar:
 *
 * This is a basic byte in an UTF-8 encoded string.
 * It's unsigned allowing to pinpoint case where char * are assigned
 * to xmlChar * (possibly making serialization back impossible).
 */
pub type xmlChar = libc::c_uchar;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut libc::c_void,
    pub elements: *mut libc::c_void,
    pub attributes: *mut libc::c_void,
    pub entities: *mut libc::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut libc::c_void,
}
pub type xmlElementType = libc::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
pub type xmlNsType = xmlElementType;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut libc::c_void,
}
pub type xmlAttributeType = libc::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct esp {
    pub hmac: *mut HMAC_CTX,
    pub cipher: *mut EVP_CIPHER_CTX,
    pub seq_backlog: uint64_t,
    pub seq: uint64_t,
    pub spi: uint32_t,
    pub enc_key: [libc::c_uchar; 64],
    pub hmac_key: [libc::c_uchar; 64],
    pub iv: [libc::c_uchar; 16],
}
/* Copyright (C) 1999-2003, 2005-2006 Free Software Foundation, Inc.
   This file is part of the GNU LIBICONV Library.

   The GNU LIBICONV Library is free software; you can redistribute it
   and/or modify it under the terms of the GNU Library General Public
   License as published by the Free Software Foundation; either version 2
   of the License, or (at your option) any later version.

   The GNU LIBICONV Library is distributed in the hope that it will be
   useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.

   You should have received a copy of the GNU Library General Public
   License along with the GNU LIBICONV Library; see the file COPYING.LIB.
   If not, write to the Free Software Foundation, Inc., 51 Franklin Street,
   Fifth Floor, Boston, MA 02110-1301, USA.  */
/* When installed, this file is called "iconv.h". */
/* version number: (major<<8) + minor */
/* Likewise */
/* We would like to #include any system header file which could define
   iconv_t, 1. in order to eliminate the risk that the user gets compilation
   errors because some other system header file includes /usr/include/iconv.h
   which defines iconv_t or declares iconv after this file, 2. when compiling
   for LIBICONV_PLUG, we need the proper iconv_t type in order to produce
   binary compatible code.
   But gcc's #include_next is not portable. Thus, once libiconv's iconv.h
   has been installed in /usr/local/include, there is no way any more to
   include the original /usr/include/iconv.h. We simply have to get away
   without it.
   Ad 1. The risk that a system header file does
   #include "iconv.h"  or  #include_next "iconv.h"
   is small. They all do #include <iconv.h>.
   Ad 2. The iconv_t type is a pointer type in all cases I have seen. (It
   has to be a scalar type because (iconv_t)(-1) is a possible return value
   from iconv_open().) */
/* Define iconv_t ourselves. */
pub type iconv_t = *mut libc::c_void;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct vpn_proto {
    pub name: *const libc::c_char,
    pub pretty_name: *const libc::c_char,
    pub description: *const libc::c_char,
    pub udp_protocol: *const libc::c_char,
    pub flags: libc::c_uint,
    pub vpn_close_session: Option<unsafe extern "C" fn(_:
                                                           *mut openconnect_info,
                                                       _: *const libc::c_char)
                                      -> libc::c_int>,
    pub obtain_cookie: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                  -> libc::c_int>,
    pub tcp_connect: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                -> libc::c_int>,
    pub tcp_mainloop: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                  _: *mut libc::c_int,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
    pub add_http_headers: Option<unsafe extern "C" fn(_:
                                                          *mut openconnect_info,
                                                      _: *mut oc_text_buf)
                                     -> ()>,
    pub udp_setup: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                               _: libc::c_int)
                              -> libc::c_int>,
    pub udp_mainloop: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                  _: *mut libc::c_int,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
    pub udp_close: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                              -> ()>,
    pub udp_shutdown: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                 -> ()>,
    pub udp_send_probes: Option<unsafe extern "C" fn(_: *mut openconnect_info)
                                    -> libc::c_int>,
    pub udp_catch_probe: Option<unsafe extern "C" fn(_: *mut openconnect_info,
                                                     _: *mut pkt)
                                    -> libc::c_int>,
}
pub type z_streamp = *mut z_stream;
#[derive ( Copy, Clone )]
#[repr(C, packed)]
pub struct oc_packed_uint16_t {
    pub d: uint16_t,
}
#[derive ( Copy, Clone )]
#[repr(C, packed)]
pub struct oc_packed_uint32_t {
    pub d: uint32_t,
}
#[inline]
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !0x7fi32 == 0i32) as libc::c_int;
}
/* USE_ASCII */
#[inline]
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t,
                              mut _f: libc::c_ulong) -> libc::c_int {
    /* USE_ASCII */
    return if isascii(_c) != 0 {
               (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong &
                    _f != 0) as libc::c_int
           } else { (__maskrune(_c, _f) != 0) as libc::c_int };
    /* USE_ASCII */
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isprint(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x40000i64 as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn _OSSwapInt32(mut _data: __uint32_t) -> __uint32_t {
    return _data.swap_bytes();
}
#[inline]
unsafe extern "C" fn _OSSwapInt16(mut _data: __uint16_t) -> __uint16_t {
    return ((_data as libc::c_int) << 8i32 | _data as libc::c_int >> 8i32) as
               __uint16_t;
}
#[inline]
unsafe extern "C" fn load_be16(mut _p: *const libc::c_void) -> uint16_t {
    let mut p: *const oc_packed_uint16_t = _p as *const oc_packed_uint16_t;
    return (if 0 != 0 {
                (((*p).d as libc::c_int & 0xff00i32) >> 8i32 |
                     ((*p).d as libc::c_int & 0xffi32) << 8i32) as __uint16_t
                    as libc::c_int
            } else { _OSSwapInt16((*p).d) as libc::c_int }) as __uint16_t;
}
#[inline]
unsafe extern "C" fn load_be32(mut _p: *const libc::c_void) -> uint32_t {
    let mut p: *const oc_packed_uint32_t = _p as *const oc_packed_uint32_t;
    return if 0 != 0 {
               ((*p).d & 0xff000000u32) >> 24i32 |
                   ((*p).d & 0xff0000i32 as libc::c_uint) >> 8i32 |
                   ((*p).d & 0xff00i32 as libc::c_uint) << 8i32 |
                   ((*p).d & 0xffi32 as libc::c_uint) << 24i32
           } else { _OSSwapInt32((*p).d) };
}
#[inline]
unsafe extern "C" fn store_be32(mut _p: *mut libc::c_void, mut d: uint32_t) {
    let mut p: *mut oc_packed_uint32_t = _p as *mut oc_packed_uint32_t;
    (*p).d =
        if 0 != 0 {
            (d & 0xff000000u32) >> 24i32 |
                (d & 0xff0000i32 as libc::c_uint) >> 8i32 |
                (d & 0xff00i32 as libc::c_uint) << 8i32 |
                (d & 0xffi32 as libc::c_uint) << 24i32
        } else { _OSSwapInt32(d) };
}
#[inline]
unsafe extern "C" fn store_be16(mut _p: *mut libc::c_void, mut d: uint16_t) {
    let mut p: *mut oc_packed_uint16_t = _p as *mut oc_packed_uint16_t;
    (*p).d =
        (if 0 != 0 {
             ((d as libc::c_int & 0xff00i32) >> 8i32 |
                  (d as libc::c_int & 0xffi32) << 8i32) as __uint16_t as
                 libc::c_int
         } else { _OSSwapInt16(d) as libc::c_int }) as __uint16_t;
}
#[inline]
unsafe extern "C" fn queue_packet(mut q: *mut pkt_q, mut p: *mut pkt)
 -> libc::c_int {
    *(*q).tail = p;
    (*p).next = 0 as *mut pkt;
    (*q).tail = &mut (*p).next;
    (*q).count += 1;
    return (*q).count;
}
#[inline]
unsafe extern "C" fn dequeue_packet(mut q: *mut pkt_q) -> *mut pkt {
    let mut ret: *mut pkt = (*q).head;
    if !ret.is_null() {
        (*q).head = (*ret).next;
        (*q).count -= 1;
        if (*q).count == 0 { (*q).tail = &mut (*q).head }
    }
    return ret;
}
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2015 Intel Corporation.
 * Copyright © 2008 Nick Andrew <nick@nick-andrew.net>
 *
 * Author: David Woodhouse <dwmw2@infradead.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * version 2.1, as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 */
/*
 * Data packets are encapsulated in the SSL stream as follows:
 *
 * 0000: Magic "STF\x1"
 * 0004: Big-endian 16-bit length (not including 8-byte header)
 * 0006: Byte packet type (see openconnect-internal.h)
 * 0008: data payload
 */
static mut data_hdr: [libc::c_char; 8] =
    ['S' as i32 as libc::c_char, 'T' as i32 as libc::c_char,
     'F' as i32 as libc::c_char, 1i32 as libc::c_char, 0i32 as libc::c_char,
     0i32 as libc::c_char, 0i32 as libc::c_char, 0i32 as libc::c_char];
/* Strange initialisers here to work around GCC PR#10676 (which was
 * fixed in GCC 4.6 but it takes a while for some systems to catch
 * up. */
static mut keepalive_pkt: pkt =
    {
        let mut init =
            pkt{len: 0,
                next: 0 as *const pkt as *mut pkt,
                c2rust_unnamed:
                    C2RustUnnamed{cstp:
                                      {
                                          let mut init =
                                              C2RustUnnamed_2{pad: [0; 16],
                                                              hdr:
                                                                  ['S' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   'T' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   'F' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   1i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar,
                                                                   7i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar],};
                                          init
                                      },},
                data: [],};
        init
    };
static mut dpd_pkt: pkt =
    {
        let mut init =
            pkt{len: 0,
                next: 0 as *const pkt as *mut pkt,
                c2rust_unnamed:
                    C2RustUnnamed{cstp:
                                      {
                                          let mut init =
                                              C2RustUnnamed_2{pad: [0; 16],
                                                              hdr:
                                                                  ['S' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   'T' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   'F' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   1i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar,
                                                                   3i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar],};
                                          init
                                      },},
                data: [],};
        init
    };
static mut dpd_resp_pkt: pkt =
    {
        let mut init =
            pkt{len: 0,
                next: 0 as *const pkt as *mut pkt,
                c2rust_unnamed:
                    C2RustUnnamed{cstp:
                                      {
                                          let mut init =
                                              C2RustUnnamed_2{pad: [0; 16],
                                                              hdr:
                                                                  ['S' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   'T' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   'F' as i32
                                                                       as
                                                                       libc::c_uchar,
                                                                   1i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar,
                                                                   4i32 as
                                                                       libc::c_uchar,
                                                                   0i32 as
                                                                       libc::c_uchar],};
                                          init
                                      },},
                data: [],};
        init
    };
/* Calculate MTU to request. Old servers simply use the X-CSTP-MTU: header,
 * which represents the tunnel MTU, while new servers do calculations on the
 * X-CSTP-Base-MTU: header which represents the link MTU between client
 * and server.
 *
 * If possible, the legacy MTU value should be the TCP MSS less 5 bytes of
 * TLS and 8 bytes of CSTP overhead. We can get the MSS from either the
 * TCP_INFO or TCP_MAXSEG sockopts.
 *
 * The base MTU comes from the TCP_INFO sockopt under Linux, but I don't know
 * how to work it out on other systems. So leave it blank and do things the
 * legacy way there. Contributions welcome...
 *
 * If we don't even have TCP_MAXSEG, then default to sending a legacy MTU of
 * 1406 which is what we always used to do.
 */
unsafe extern "C" fn calculate_mtu(mut vpninfo: *mut openconnect_info,
                                   mut base_mtu: *mut libc::c_int,
                                   mut mtu: *mut libc::c_int) {
    *mtu = (*vpninfo).reqmtu;
    *base_mtu = (*vpninfo).basemtu;
    if *base_mtu == 0 {
        /* Default */
        *base_mtu = 1406i32
    }
    if *base_mtu < 1280i32 { *base_mtu = 1280i32 }
    if *mtu == 0 {
        /* remove IP/UDP and DTLS overhead from base MTU to calculate tunnel MTU */
        *mtu = *base_mtu - (1i32 + 13i32 + 20i32 + 32i32 + 16i32) - 8i32;
        if (*(*vpninfo).peer_addr).sa_family as libc::c_int == 30i32 {
            *mtu -= 40i32
        } else { *mtu -= 20i32 }
    };
}
unsafe extern "C" fn append_compr_types(mut buf: *mut oc_text_buf,
                                        mut proto: *const libc::c_char,
                                        mut avail: libc::c_int) {
    if avail != 0 {
        let mut sep: libc::c_char = ' ' as i32 as libc::c_char;
        buf_append(buf,
                   b"X-%s-Accept-Encoding:\x00" as *const u8 as
                       *const libc::c_char, proto);
        if avail & 1i32 << 2i32 != 0 {
            buf_append(buf,
                       b"%coc-lz4\x00" as *const u8 as *const libc::c_char,
                       sep as libc::c_int);
            sep = ',' as i32 as libc::c_char
        }
        if avail & 1i32 << 1i32 != 0 {
            buf_append(buf, b"%clzs\x00" as *const u8 as *const libc::c_char,
                       sep as libc::c_int);
            sep = ',' as i32 as libc::c_char
        }
        if avail & 1i32 << 0i32 != 0 {
            buf_append(buf,
                       b"%cdeflate\x00" as *const u8 as *const libc::c_char,
                       sep as libc::c_int);
            sep = ',' as i32 as libc::c_char
        }
        buf_append(buf, b"\r\n\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn append_mobile_headers(mut vpninfo: *mut openconnect_info,
                                           mut buf: *mut oc_text_buf) {
    if !(*vpninfo).mobile_platform_version.is_null() {
        buf_append(buf,
                   b"X-AnyConnect-Identifier-ClientVersion: %s\r\n\x00" as
                       *const u8 as *const libc::c_char,
                   if !(*vpninfo).version_string.is_null() {
                       (*vpninfo).version_string
                   } else { openconnect_version_str });
        buf_append(buf,
                   b"X-AnyConnect-Identifier-Platform: %s\r\n\x00" as
                       *const u8 as *const libc::c_char, (*vpninfo).platname);
        buf_append(buf,
                   b"X-AnyConnect-Identifier-PlatformVersion: %s\r\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*vpninfo).mobile_platform_version);
        buf_append(buf,
                   b"X-AnyConnect-Identifier-DeviceType: %s\r\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*vpninfo).mobile_device_type);
        buf_append(buf,
                   b"X-AnyConnect-Identifier-Device-UniqueID: %s\r\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*vpninfo).mobile_device_uniqueid);
    };
}
unsafe extern "C" fn parse_hex_val(mut str: *const libc::c_char,
                                   mut storage: *mut libc::c_uchar,
                                   mut max_storage_len: libc::c_uint,
                                   mut changed: *mut libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = strlen(str) as libc::c_int;
    let mut i: libc::c_uint = 0;
    if len % 2i32 == 1i32 ||
           len as libc::c_uint >
               (2i32 as libc::c_uint).wrapping_mul(max_storage_len) {
        return -22i32
    }
    i = 0i32 as libc::c_uint;
    while i < len as libc::c_uint {
        let mut c: libc::c_uchar = unhex(str.offset(i as isize));
        if *storage.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as
               libc::c_int != c as libc::c_int {
            *storage.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) =
                c;
            *changed = 1i32
        }
        i = i.wrapping_add(2i32 as libc::c_uint)
    }
    return len / 2i32;
}
unsafe extern "C" fn start_cstp_connection(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut reqbuf: *mut oc_text_buf = 0 as *mut oc_text_buf;
    let mut buf: [libc::c_char; 65536] = [0; 65536];
    let mut i: libc::c_int = 0;
    let mut dtls_secret_set: libc::c_int = 0i32;
    let mut retried: libc::c_int = 0i32;
    let mut sessid_found: libc::c_int = 0i32;
    let mut next_dtls_option: *mut *mut oc_vpn_option =
        &mut (*vpninfo).dtls_options;
    let mut next_cstp_option: *mut *mut oc_vpn_option =
        &mut (*vpninfo).cstp_options;
    let mut old_cstp_opts: *mut oc_vpn_option = (*vpninfo).cstp_options;
    let mut old_dtls_opts: *mut oc_vpn_option = (*vpninfo).dtls_options;
    let mut old_addr: *const libc::c_char = (*vpninfo).ip_info.addr;
    let mut old_netmask: *const libc::c_char = (*vpninfo).ip_info.netmask;
    let mut old_addr6: *const libc::c_char = (*vpninfo).ip_info.addr6;
    let mut old_netmask6: *const libc::c_char = (*vpninfo).ip_info.netmask6;
    let mut base_mtu: libc::c_int = 0i32;
    let mut mtu: libc::c_int = 0i32;
    /* Clear old options which will be overwritten */
    (*vpninfo).ip_info.netmask = 0 as *const libc::c_char;
    (*vpninfo).ip_info.addr = (*vpninfo).ip_info.netmask;
    (*vpninfo).ip_info.netmask6 = 0 as *const libc::c_char;
    (*vpninfo).ip_info.addr6 = (*vpninfo).ip_info.netmask6;
    (*vpninfo).dtls_options = 0 as *mut oc_vpn_option;
    (*vpninfo).cstp_options = (*vpninfo).dtls_options;
    (*vpninfo).ip_info.proxy_pac = 0 as *const libc::c_char;
    (*vpninfo).ip_info.domain = (*vpninfo).ip_info.proxy_pac;
    (*vpninfo).banner = 0 as *const libc::c_char;
    i = 0i32;
    while i < 3i32 {
        (*vpninfo).ip_info.nbns[i as usize] = 0 as *const libc::c_char;
        (*vpninfo).ip_info.dns[i as usize] =
            (*vpninfo).ip_info.nbns[i as usize];
        i += 1
    }
    free_split_routes(vpninfo);
    loop  {
        calculate_mtu(vpninfo, &mut base_mtu, &mut mtu);
        (*vpninfo).cstp_basemtu = base_mtu;
        reqbuf = buf_alloc();
        buf_append(reqbuf,
                   b"CONNECT /CSCOSSLC/tunnel HTTP/1.1\r\n\x00" as *const u8
                       as *const libc::c_char);
        if (*vpninfo).port != 443i32 {
            buf_append(reqbuf,
                       b"Host: %s:%d\r\n\x00" as *const u8 as
                           *const libc::c_char, (*vpninfo).hostname,
                       (*vpninfo).port);
        } else {
            buf_append(reqbuf,
                       b"Host: %s\r\n\x00" as *const u8 as
                           *const libc::c_char, (*vpninfo).hostname);
        }
        buf_append(reqbuf,
                   b"User-Agent: %s\r\n\x00" as *const u8 as
                       *const libc::c_char, (*vpninfo).useragent);
        buf_append(reqbuf,
                   b"Cookie: webvpn=%s\r\n\x00" as *const u8 as
                       *const libc::c_char, (*vpninfo).cookie);
        buf_append(reqbuf,
                   b"X-CSTP-Version: 1\r\n\x00" as *const u8 as
                       *const libc::c_char);
        buf_append(reqbuf,
                   b"X-CSTP-Hostname: %s\r\n\x00" as *const u8 as
                       *const libc::c_char, (*vpninfo).localname);
        append_mobile_headers(vpninfo, reqbuf);
        append_compr_types(reqbuf,
                           b"CSTP\x00" as *const u8 as *const libc::c_char,
                           (*vpninfo).req_compr);
        buf_append(reqbuf,
                   b"X-CSTP-Base-MTU: %d\r\n\x00" as *const u8 as
                       *const libc::c_char, base_mtu);
        if mtu != 0 {
            buf_append(reqbuf,
                       b"X-CSTP-MTU: %d\r\n\x00" as *const u8 as
                           *const libc::c_char, mtu);
        }
        buf_append(reqbuf,
                   b"X-CSTP-Address-Type: %s\r\n\x00" as *const u8 as
                       *const libc::c_char,
                   if (*vpninfo).disable_ipv6 != 0 {
                       b"IPv4\x00" as *const u8 as *const libc::c_char
                   } else {
                       b"IPv6,IPv4\x00" as *const u8 as *const libc::c_char
                   });
        if (*vpninfo).disable_ipv6 == 0 {
            buf_append(reqbuf,
                       b"X-CSTP-Full-IPv6-Capability: true\r\n\x00" as
                           *const u8 as *const libc::c_char);
        }
        if (*vpninfo).dtls_state != 2i32 {
            /* The X-DTLS-Master-Secret is only used for the legacy protocol negotation
		 * which required the client to send explicitly the secret. In the PSK-NEGOTIATE
		 * method, the master secret is implicitly agreed on */
            buf_append(reqbuf,
                       b"X-DTLS-Master-Secret: \x00" as *const u8 as
                           *const libc::c_char);
            i = 0i32;
            while (i as libc::c_ulong) <
                      ::std::mem::size_of::<[libc::c_uchar; 48]>() as
                          libc::c_ulong {
                buf_append(reqbuf,
                           b"%02X\x00" as *const u8 as *const libc::c_char,
                           (*vpninfo).dtls_secret[i as usize] as libc::c_int);
                dtls_secret_set |=
                    (*vpninfo).dtls_secret[i as usize] as libc::c_int;
                i += 1
            }
            buf_append(reqbuf,
                       b"\r\n\x00" as *const u8 as *const libc::c_char);
            if dtls_secret_set == 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"CRITICAL ERROR: DTLS master secret is uninitialised. Please report this.\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                buf_free(reqbuf);
                return -22i32
            }
            if !(*vpninfo).dtls_ciphers.is_null() ||
                   !(*vpninfo).dtls12_ciphers.is_null() {
                if !(*vpninfo).dtls_ciphers.is_null() {
                    buf_append(reqbuf,
                               b"X-DTLS-CipherSuite: %s\r\n\x00" as *const u8
                                   as *const libc::c_char,
                               (*vpninfo).dtls_ciphers);
                }
                if !(*vpninfo).dtls12_ciphers.is_null() {
                    buf_append(reqbuf,
                               b"X-DTLS12-CipherSuite: %s\r\n\x00" as
                                   *const u8 as *const libc::c_char,
                               (*vpninfo).dtls12_ciphers);
                }
            } else {
                let mut dtls_cl: *mut oc_text_buf = 0 as *mut oc_text_buf;
                let mut dtls12_cl: *mut oc_text_buf = 0 as *mut oc_text_buf;
                dtls_cl = buf_alloc();
                dtls12_cl = buf_alloc();
                gather_dtls_ciphers(vpninfo, dtls_cl, dtls12_cl);
                if buf_error(dtls_cl) == 0 && (*dtls_cl).pos != 0 {
                    buf_append(reqbuf,
                               b"X-DTLS-CipherSuite: %s\r\n\x00" as *const u8
                                   as *const libc::c_char, (*dtls_cl).data);
                }
                if buf_error(dtls12_cl) == 0 && (*dtls12_cl).pos != 0 {
                    buf_append(reqbuf,
                               b"X-DTLS12-CipherSuite: %s\r\n\x00" as
                                   *const u8 as *const libc::c_char,
                               (*dtls12_cl).data);
                }
                buf_free(dtls_cl);
                buf_free(dtls12_cl);
            }
            append_compr_types(reqbuf,
                               b"DTLS\x00" as *const u8 as
                                   *const libc::c_char,
                               (*vpninfo).req_compr & !(1i32 << 0i32));
        }
        buf_append(reqbuf, b"\r\n\x00" as *const u8 as *const libc::c_char);
        if buf_error(reqbuf) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Error creating HTTPS CONNECT request\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            return buf_free(reqbuf)
        }
        if (*vpninfo).dump_http_traffic != 0 {
            dump_buf(vpninfo, '>' as i32 as libc::c_char, (*reqbuf).data);
        }
        (*vpninfo).ssl_write.expect("non-null function pointer")(vpninfo,
                                                                 (*reqbuf).data,
                                                                 (*reqbuf).pos
                                                                     as
                                                                     size_t);
        buf_free(reqbuf);
        /* FIXME: Use process_http_response() instead of reimplementing it. It has
	   a header callback function, and can cope with CONNECT requests. */
        i =
            (*vpninfo).ssl_gets.expect("non-null function pointer")(vpninfo,
                                                                    buf.as_mut_ptr(),
                                                                    65536i32
                                                                        as
                                                                        size_t);
        if i < 0i32 {
            if i == -4i32 { return i }
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Error fetching HTTPS response\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            if retried == 0 {
                retried = 1i32;
                openconnect_close_https(vpninfo, 0i32);
                if openconnect_open_https(vpninfo) != 0 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Failed to open HTTPS connection to %s\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).hostname);
                    }
                    return -5i32
                }
            } else { return -22i32 }
        } else {
            if strncmp(buf.as_mut_ptr(),
                       b"HTTP/1.1 200 \x00" as *const u8 as
                           *const libc::c_char, 13i32 as libc::c_ulong) != 0 {
                if strncmp(buf.as_mut_ptr(),
                           b"HTTP/1.1 503 \x00" as *const u8 as
                               *const libc::c_char, 13i32 as libc::c_ulong) ==
                       0 {
                    /* "Service Unavailable. Why? */
                    let mut reason: *const libc::c_char =
                        b"<unknown>\x00" as *const u8 as *const libc::c_char;
                    loop  {
                        i =
                            (*vpninfo).ssl_gets.expect("non-null function pointer")(vpninfo,
                                                                                    buf.as_mut_ptr(),
                                                                                    ::std::mem::size_of::<[libc::c_char; 65536]>()
                                                                                        as
                                                                                        libc::c_ulong);
                        if !(i != 0) { break ; }
                        if !(strncmp(buf.as_mut_ptr(),
                                     b"X-Reason: \x00" as *const u8 as
                                         *const libc::c_char,
                                     10i32 as libc::c_ulong) == 0) {
                            continue ;
                        }
                        reason = buf.as_mut_ptr().offset(10);
                        break ;
                    }
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"VPN service unavailable; reason: %s\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                reason);
                    }
                    return -22i32
                }
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Got inappropriate HTTP CONNECT response: %s\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            buf.as_mut_ptr());
                }
                if strncmp(buf.as_mut_ptr(),
                           b"HTTP/1.1 401 \x00" as *const u8 as
                               *const libc::c_char, 13i32 as libc::c_ulong) ==
                       0 {
                    return -1i32
                }
                return -22i32
            }
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Got CONNECT response: %s\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        buf.as_mut_ptr());
            }
            /* We may have advertised it, but we only do it if the server agrees */
            (*vpninfo).dtls_compr = 0i32;
            (*vpninfo).cstp_compr = (*vpninfo).dtls_compr;
            mtu = 0i32;
            loop  {
                i =
                    (*vpninfo).ssl_gets.expect("non-null function pointer")(vpninfo,
                                                                            buf.as_mut_ptr(),
                                                                            ::std::mem::size_of::<[libc::c_char; 65536]>()
                                                                                as
                                                                                libc::c_ulong);
                if !(i != 0) { break ; }
                let mut new_option: *mut oc_vpn_option =
                    0 as *mut oc_vpn_option;
                let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
                if i < 0i32 { return i }
                colon = strchr(buf.as_mut_ptr(), ':' as i32);
                if colon.is_null() { continue ; }
                *colon = 0i32 as libc::c_char;
                colon = colon.offset(1);
                if *colon as libc::c_int == ' ' as i32 {
                    colon = colon.offset(1)
                }
                if strncmp(buf.as_mut_ptr(),
                           b"X-DTLS-\x00" as *const u8 as *const libc::c_char,
                           7i32 as libc::c_ulong) != 0 &&
                       strncmp(buf.as_mut_ptr(),
                               b"X-CSTP-\x00" as *const u8 as
                                   *const libc::c_char, 7i32 as libc::c_ulong)
                           != 0 &&
                       strncmp(buf.as_mut_ptr(),
                               b"X-DTLS12-\x00" as *const u8 as
                                   *const libc::c_char, 9i32 as libc::c_ulong)
                           != 0 {
                    continue ;
                }
                new_option =
                    malloc(::std::mem::size_of::<oc_vpn_option>() as
                               libc::c_ulong) as *mut oc_vpn_option;
                if new_option.is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"No memory for options\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    return -12i32
                }
                (*new_option).option = strdup(buf.as_mut_ptr());
                (*new_option).value = strdup(colon);
                (*new_option).next = 0 as *mut oc_vpn_option;
                if (*new_option).option.is_null() ||
                       (*new_option).value.is_null() {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"No memory for options\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    free((*new_option).option as *mut libc::c_void);
                    free((*new_option).value as *mut libc::c_void);
                    free(new_option as *mut libc::c_void);
                    return -12i32
                }
                /* This contains the whole document, including the webvpn cookie. */
                if strcasecmp(buf.as_mut_ptr(),
                              b"X-CSTP-Post-Auth-XML\x00" as *const u8 as
                                  *const libc::c_char) == 0 {
                    if (*vpninfo).verbose >= 2i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                2i32,
                                                                                b"%s: %s\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                buf.as_mut_ptr(),
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"<elided>\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                } else if (*vpninfo).verbose >= 2i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            2i32,
                                                                            b"%s: %s\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char,
                                                                            buf.as_mut_ptr(),
                                                                            colon);
                }
                i = 7i32;
                if i != 0 &&
                       strncmp(buf.as_mut_ptr(),
                               b"X-DTLS-\x00" as *const u8 as
                                   *const libc::c_char, 7i32 as libc::c_ulong)
                           == 0 ||
                       {
                           i = 9i32;
                           i != 0 &&
                               strncmp(buf.as_mut_ptr(),
                                       b"X-DTLS12-\x00" as *const u8 as
                                           *const libc::c_char,
                                       9i32 as libc::c_ulong) == 0
                       } {
                    *next_dtls_option = new_option;
                    next_dtls_option = &mut (*new_option).next;
                    if strcmp(buf.as_mut_ptr().offset(i as isize),
                              b"MTU\x00" as *const u8 as *const libc::c_char)
                           == 0 {
                        let mut dtlsmtu: libc::c_int =
                            atol(colon) as libc::c_int;
                        if dtlsmtu > mtu { mtu = dtlsmtu }
                    } else if strcmp(buf.as_mut_ptr().offset(i as isize),
                                     b"Session-ID\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        let mut dtls_sessid_changed: libc::c_int = 0i32;
                        let mut vsize: libc::c_int = 0;
                        vsize =
                            parse_hex_val(colon,
                                          (*vpninfo).dtls_session_id.as_mut_ptr(),
                                          ::std::mem::size_of::<[libc::c_uchar; 32]>()
                                              as libc::c_ulong as
                                              libc::c_uint,
                                          &mut dtls_sessid_changed);
                        if vsize != 32i32 {
                            if (*vpninfo).verbose >= 0i32 {
                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                        0i32,
                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char,
                                                                                                         b"X-DTLS-Session-ID not 64 characters; is: \"%s\"\n\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char),
                                                                                        colon);
                            }
                            (*vpninfo).dtls_attempt_period = 0i32;
                            return -22i32
                        }
                        sessid_found = 1i32;
                        if dtls_sessid_changed != 0 &&
                               (*vpninfo).dtls_state > 3i32 {
                            (*vpninfo).dtls_need_reconnect = 1i32
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(i as isize),
                                     b"App-ID\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        let mut dtls_appid_changed: libc::c_int = 0i32;
                        let mut vsize_0: libc::c_int = 0;
                        vsize_0 =
                            parse_hex_val(colon,
                                          (*vpninfo).dtls_app_id.as_mut_ptr(),
                                          ::std::mem::size_of::<[libc::c_uchar; 32]>()
                                              as libc::c_ulong as
                                              libc::c_uint,
                                          &mut dtls_appid_changed);
                        if vsize_0 <= 0i32 {
                            if (*vpninfo).verbose >= 0i32 {
                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                        0i32,
                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char,
                                                                                                         b"X-DTLS-Session-ID is invalid; is: \"%s\"\n\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char),
                                                                                        colon);
                            }
                            (*vpninfo).dtls_attempt_period = 0i32;
                            return -22i32
                        }
                        (*vpninfo).dtls_app_id_size = vsize_0 as libc::c_uint;
                        sessid_found = 1i32;
                        if dtls_appid_changed != 0 &&
                               (*vpninfo).dtls_state > 3i32 {
                            (*vpninfo).dtls_need_reconnect = 1i32
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(i as isize),
                                     b"Content-Encoding\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        if strcmp(colon,
                                  b"lzs\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            (*vpninfo).dtls_compr = 1i32 << 1i32
                        } else if strcmp(colon,
                                         b"oc-lz4\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            (*vpninfo).dtls_compr = 1i32 << 2i32
                        } else {
                            if (*vpninfo).verbose >= 0i32 {
                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                        0i32,
                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char,
                                                                                                         b"Unknown DTLS-Content-Encoding %s\n\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char),
                                                                                        colon);
                            }
                            return -22i32
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(i as isize),
                                     b"CipherSuite\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        /* Remember if it came from a 'X-DTLS12-CipherSuite:' header */
                        (*vpninfo).cisco_dtls12 = (i == 9i32) as libc::c_int;
                        (*vpninfo).dtls_cipher = strdup(colon)
                    }
                } else {
                    /* CSTP options... */
                    *next_cstp_option = new_option;
                    next_cstp_option = &mut (*new_option).next;
                    if strcmp(buf.as_mut_ptr().offset(7),
                              b"Keepalive\x00" as *const u8 as
                                  *const libc::c_char) == 0 {
                        (*vpninfo).ssl_times.keepalive =
                            atol(colon) as libc::c_int
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Idle-Timeout\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        (*vpninfo).idle_timeout = atol(colon) as libc::c_int
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"DPD\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        let mut j: libc::c_int = atol(colon) as libc::c_int;
                        if j != 0 &&
                               ((*vpninfo).ssl_times.dpd == 0 ||
                                    j < (*vpninfo).ssl_times.dpd) {
                            (*vpninfo).ssl_times.dpd = j
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Rekey-Time\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        (*vpninfo).ssl_times.rekey =
                            atol(colon) as libc::c_int
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Rekey-Method\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        if strcmp(colon,
                                  b"new-tunnel\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            (*vpninfo).ssl_times.rekey_method = 1i32
                        } else if strcmp(colon,
                                         b"ssl\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            (*vpninfo).ssl_times.rekey_method = 2i32
                        } else { (*vpninfo).ssl_times.rekey_method = 0i32 }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Content-Encoding\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        if strcmp(colon,
                                  b"deflate\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            (*vpninfo).cstp_compr = 1i32 << 0i32
                        } else if strcmp(colon,
                                         b"lzs\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            (*vpninfo).cstp_compr = 1i32 << 1i32
                        } else if strcmp(colon,
                                         b"oc-lz4\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                            (*vpninfo).cstp_compr = 1i32 << 2i32
                        } else {
                            if (*vpninfo).verbose >= 0i32 {
                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                        0i32,
                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char,
                                                                                                         b"Unknown CSTP-Content-Encoding %s\n\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char),
                                                                                        colon);
                            }
                            return -22i32
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Base-MTU\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        (*vpninfo).cstp_basemtu = atol(colon) as libc::c_int
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"MTU\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        let mut cstpmtu: libc::c_int =
                            atol(colon) as libc::c_int;
                        if cstpmtu > mtu { mtu = cstpmtu }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"DynDNS\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        if strcmp(colon,
                                  b"true\x00" as *const u8 as
                                      *const libc::c_char) == 0 {
                            (*vpninfo).is_dyndns = 1i32
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Address-IP6\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        (*vpninfo).ip_info.netmask6 = (*new_option).value
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Address\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        if !strchr((*new_option).value, ':' as i32).is_null()
                           {
                            if (*vpninfo).disable_ipv6 == 0 {
                                (*vpninfo).ip_info.addr6 = (*new_option).value
                            }
                        } else {
                            (*vpninfo).ip_info.addr = (*new_option).value
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Netmask\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        if !strchr((*new_option).value, ':' as i32).is_null()
                           {
                            if (*vpninfo).disable_ipv6 == 0 {
                                (*vpninfo).ip_info.netmask6 =
                                    (*new_option).value
                            }
                        } else {
                            (*vpninfo).ip_info.netmask = (*new_option).value
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"DNS\x00" as *const u8 as
                                         *const libc::c_char) == 0 ||
                                  strcmp(buf.as_mut_ptr().offset(7),
                                         b"DNS-IP6\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                        let mut j_0: libc::c_int = 0;
                        j_0 = 0i32;
                        while j_0 < 3i32 {
                            if (*vpninfo).ip_info.dns[j_0 as usize].is_null()
                               {
                                (*vpninfo).ip_info.dns[j_0 as usize] =
                                    (*new_option).value;
                                break ;
                            } else { j_0 += 1 }
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"NBNS\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        let mut j_1: libc::c_int = 0;
                        j_1 = 0i32;
                        while j_1 < 3i32 {
                            if (*vpninfo).ip_info.nbns[j_1 as usize].is_null()
                               {
                                (*vpninfo).ip_info.nbns[j_1 as usize] =
                                    (*new_option).value;
                                break ;
                            } else { j_1 += 1 }
                        }
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Default-Domain\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        (*vpninfo).ip_info.domain = (*new_option).value
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"MSIE-Proxy-PAC-URL\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        (*vpninfo).ip_info.proxy_pac = (*new_option).value
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Banner\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        (*vpninfo).banner = (*new_option).value
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Split-DNS\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                        let mut dns: *mut oc_split_include =
                            malloc(::std::mem::size_of::<oc_split_include>()
                                       as libc::c_ulong) as
                                *mut oc_split_include;
                        if dns.is_null() { continue ; }
                        (*dns).route = (*new_option).value;
                        (*dns).next = (*vpninfo).ip_info.split_dns;
                        (*vpninfo).ip_info.split_dns = dns
                    } else if strcmp(buf.as_mut_ptr().offset(7),
                                     b"Split-Include\x00" as *const u8 as
                                         *const libc::c_char) == 0 ||
                                  strcmp(buf.as_mut_ptr().offset(7),
                                         b"Split-Include-IP6\x00" as *const u8
                                             as *const libc::c_char) == 0 {
                        let mut inc: *mut oc_split_include =
                            malloc(::std::mem::size_of::<oc_split_include>()
                                       as libc::c_ulong) as
                                *mut oc_split_include;
                        if inc.is_null() { continue ; }
                        (*inc).route = (*new_option).value;
                        (*inc).next = (*vpninfo).ip_info.split_includes;
                        (*vpninfo).ip_info.split_includes = inc
                    } else {
                        if !(strcmp(buf.as_mut_ptr().offset(7),
                                    b"Split-Exclude\x00" as *const u8 as
                                        *const libc::c_char) == 0 ||
                                 strcmp(buf.as_mut_ptr().offset(7),
                                        b"Split-Exclude-IP6\x00" as *const u8
                                            as *const libc::c_char) == 0) {
                            continue ;
                        }
                        let mut exc: *mut oc_split_include =
                            malloc(::std::mem::size_of::<oc_split_include>()
                                       as libc::c_ulong) as
                                *mut oc_split_include;
                        if exc.is_null() { continue ; }
                        (*exc).route = (*new_option).value;
                        (*exc).next = (*vpninfo).ip_info.split_excludes;
                        (*vpninfo).ip_info.split_excludes = exc
                    }
                }
            }
            if mtu == 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"No MTU received. Aborting\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                return -22i32
            }
            (*vpninfo).ip_info.mtu = mtu;
            if (*vpninfo).ip_info.addr.is_null() &&
                   (*vpninfo).ip_info.addr6.is_null() &&
                   (*vpninfo).ip_info.netmask6.is_null() {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"No IP address received. Aborting\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                return -22i32
            }
            if mtu < 1280i32 &&
                   (!(*vpninfo).ip_info.addr6.is_null() ||
                        !(*vpninfo).ip_info.netmask6.is_null()) {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"IPv6 configuration received but MTU %d is too small.\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            mtu);
                }
            }
            if !old_addr.is_null() {
                if strcmp(old_addr, (*vpninfo).ip_info.addr) != 0 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Reconnect gave different Legacy IP address (%s != %s)\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).ip_info.addr,
                                                                                old_addr);
                    }
                    /* EPERM means that the retry loop will abort and won't keep trying. */
                    return -1i32
                }
            }
            if !old_netmask.is_null() {
                if strcmp(old_netmask, (*vpninfo).ip_info.netmask) != 0 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Reconnect gave different Legacy IP netmask (%s != %s)\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).ip_info.netmask,
                                                                                old_netmask);
                    }
                    return -1i32
                }
            }
            if !old_addr6.is_null() {
                if strcmp(old_addr6, (*vpninfo).ip_info.addr6) != 0 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Reconnect gave different IPv6 address (%s != %s)\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).ip_info.addr6,
                                                                                old_addr6);
                    }
                    return -1i32
                }
            }
            if !old_netmask6.is_null() {
                if strcmp(old_netmask6, (*vpninfo).ip_info.netmask6) != 0 {
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"Reconnect gave different IPv6 netmask (%s != %s)\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char),
                                                                                (*vpninfo).ip_info.netmask6,
                                                                                old_netmask6);
                    }
                    return -1i32
                }
            }
            free_optlist(old_dtls_opts);
            free_optlist(old_cstp_opts);
            if (*vpninfo).verbose >= 1i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        1i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"CSTP connected. DPD %d, Keepalive %d\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        (*vpninfo).ssl_times.dpd,
                                                                        (*vpninfo).ssl_times.keepalive);
            }
            if (*vpninfo).verbose >= 2i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        2i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"CSTP Ciphersuite: %s\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        openconnect_get_cstp_cipher(vpninfo));
            }
            if (*vpninfo)._select_nfds <= (*vpninfo).ssl_fd {
                (*vpninfo)._select_nfds = (*vpninfo).ssl_fd + 1i32
            }
            let mut __fd: libc::c_int = (*vpninfo).ssl_fd;
            (*vpninfo)._select_rfds.fds_bits[(__fd as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8i32
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] |=
                ((1i32 as libc::c_ulong) <<
                     (__fd as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                           as
                                                                                           libc::c_ulong)))
                    as __int32_t;
            let mut __fd_0: libc::c_int = (*vpninfo).ssl_fd;
            (*vpninfo)._select_efds.fds_bits[(__fd_0 as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8i32
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] |=
                ((1i32 as libc::c_ulong) <<
                     (__fd_0 as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8i32
                                                                                           as
                                                                                           libc::c_ulong)))
                    as __int32_t;
            if sessid_found == 0 { (*vpninfo).dtls_attempt_period = 0i32 }
            if (*vpninfo).ssl_times.rekey <= 0i32 {
                (*vpninfo).ssl_times.rekey_method = 0i32
            }
            (*vpninfo).ssl_times.last_tx = time(0 as *mut time_t);
            (*vpninfo).ssl_times.last_rx = (*vpninfo).ssl_times.last_tx;
            (*vpninfo).ssl_times.last_rekey = (*vpninfo).ssl_times.last_rx;
            return 0i32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cstp_connect(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut deflate_bufsize: libc::c_int = 0i32;
    let mut compr_type: libc::c_int = 0;
    /* This needs to be done before openconnect_setup_dtls() because it's
	   sent with the CSTP CONNECT handshake. Even if we don't end up doing
	   DTLS. */
    if (*vpninfo).dtls_state == 0i32 {
        if openconnect_random((*vpninfo).dtls_secret.as_mut_ptr() as
                                  *mut libc::c_void,
                              ::std::mem::size_of::<[libc::c_uchar; 48]>() as
                                  libc::c_ulong as libc::c_int) != 0 {
            return -22i32
        }
        /* The application will later call openconnect_setup_dtls() */
        (*vpninfo).dtls_state = 1i32
    }
    ret = openconnect_open_https(vpninfo);
    if ret != 0 { return ret }
    ret = start_cstp_connection(vpninfo);
    if !(ret != 0) {
        /* Allow for the theoretical possibility of having *different*
	 * compression type for CSTP and DTLS. Although all we've seen
	 * in practice is that one is enabled and the other isn't. */
        compr_type = (*vpninfo).cstp_compr | (*vpninfo).dtls_compr;
        /* This will definitely be smaller than zlib's */
        if compr_type & (1i32 << 1i32 | 1i32 << 2i32) != 0 {
            deflate_bufsize = (*vpninfo).ip_info.mtu
        }
        /* If deflate compression is enabled (which is CSTP-only), it needs its
	 * context to be allocated. */
        if compr_type & 1i32 << 0i32 != 0 {
            (*vpninfo).deflate_adler32 = 1i32 as uint32_t;
            (*vpninfo).inflate_adler32 = 1i32 as uint32_t;
            if inflateInit2_(&mut (*vpninfo).inflate_strm, -12i32,
                             b"1.2.11\x00" as *const u8 as
                                 *const libc::c_char,
                             ::std::mem::size_of::<z_stream>() as
                                 libc::c_ulong as libc::c_int) != 0 ||
                   deflateInit2_(&mut (*vpninfo).deflate_strm, -1i32, 8i32,
                                 -12i32, 9i32, 0i32,
                                 b"1.2.11\x00" as *const u8 as
                                     *const libc::c_char,
                                 ::std::mem::size_of::<z_stream>() as
                                     libc::c_ulong as libc::c_int) != 0 {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"Compression setup failed\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                }
                ret = -12i32;
                current_block = 195929901388605858;
            } else {
                /* Add four bytes for the adler32 */
                deflate_bufsize =
                    deflateBound(&mut (*vpninfo).deflate_strm,
                                 (*vpninfo).ip_info.mtu as
                                     uLong).wrapping_add(4i32 as
                                                             libc::c_ulong) as
                        libc::c_int;
                current_block = 11298138898191919651;
            }
        } else { current_block = 11298138898191919651; }
        match current_block {
            195929901388605858 => { }
            _ =>
            /* If *any* compression is enabled, we'll need a deflate_pkt to compress into */
            {
                if deflate_bufsize > (*vpninfo).deflate_pkt_size {
                    free((*vpninfo).deflate_pkt as *mut libc::c_void);
                    (*vpninfo).deflate_pkt =
                        malloc((::std::mem::size_of::<pkt>() as
                                    libc::c_ulong).wrapping_add(deflate_bufsize
                                                                    as
                                                                    libc::c_ulong))
                            as *mut pkt;
                    if (*vpninfo).deflate_pkt.is_null() {
                        (*vpninfo).deflate_pkt_size = 0i32;
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Allocation of deflate buffer failed\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        ret = -12i32
                    } else {
                        (*vpninfo).deflate_pkt_size = deflate_bufsize;
                        memset((*vpninfo).deflate_pkt as *mut libc::c_void,
                               0i32,
                               ::std::mem::size_of::<pkt>() as libc::c_ulong);
                        memcpy((*(*vpninfo).deflate_pkt).c2rust_unnamed.cstp.hdr.as_mut_ptr()
                                   as *mut libc::c_void,
                               data_hdr.as_ptr() as *const libc::c_void,
                               8i32 as libc::c_ulong);
                        (*(*vpninfo).deflate_pkt).c2rust_unnamed.cstp.hdr[6] =
                            8i32 as libc::c_uchar
                    }
                }
            }
        }
    }
    if ret < 0i32 { openconnect_close_https(vpninfo, 0i32); }
    return ret;
}
unsafe extern "C" fn cstp_reconnect(mut vpninfo: *mut openconnect_info)
 -> libc::c_int {
    if (*vpninfo).cstp_compr == 1i32 << 0i32 {
        /* Requeue the original packet that was deflated */
        if (*vpninfo).current_ssl_pkt == (*vpninfo).deflate_pkt {
            (*vpninfo).current_ssl_pkt = 0 as *mut pkt;
            queue_packet(&mut (*vpninfo).outgoing_queue,
                         (*vpninfo).pending_deflated_pkt);
            (*vpninfo).pending_deflated_pkt = 0 as *mut pkt
        }
        inflateEnd(&mut (*vpninfo).inflate_strm);
        deflateEnd(&mut (*vpninfo).deflate_strm);
    }
    return ssl_reconnect(vpninfo);
}
#[no_mangle]
pub unsafe extern "C" fn decompress_and_queue_packet(mut vpninfo:
                                                         *mut openconnect_info,
                                                     mut compr_type:
                                                         libc::c_int,
                                                     mut buf:
                                                         *mut libc::c_uchar,
                                                     mut len: libc::c_int)
 -> libc::c_int {
    /* Some servers send us packets that are larger than
	   negotiated MTU after decompression. We reserve some extra
	   space to handle that */
    let mut receive_mtu: libc::c_int =
        if 16384i32 > (*vpninfo).ip_info.mtu {
            16384i32
        } else { (*vpninfo).ip_info.mtu };
    let mut new: *mut pkt =
        malloc((::std::mem::size_of::<pkt>() as
                    libc::c_ulong).wrapping_add(receive_mtu as libc::c_ulong))
            as *mut pkt;
    let mut comprname: *const libc::c_char =
        b"\x00" as *const u8 as *const libc::c_char;
    if new.is_null() { return -12i32 }
    (*new).next = 0 as *mut pkt;
    if compr_type == 1i32 << 0i32 {
        let mut pkt_sum: uint32_t = 0;
        comprname = b"deflate\x00" as *const u8 as *const libc::c_char;
        (*vpninfo).inflate_strm.next_in = buf;
        (*vpninfo).inflate_strm.avail_in = (len - 4i32) as uInt;
        (*vpninfo).inflate_strm.next_out = (*new).data.as_mut_ptr();
        (*vpninfo).inflate_strm.avail_out = receive_mtu as uInt;
        (*vpninfo).inflate_strm.total_out = 0i32 as uLong;
        if inflate(&mut (*vpninfo).inflate_strm, 2i32) != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"inflate failed\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            free(new as *mut libc::c_void);
            return -22i32
        }
        (*new).len = (*vpninfo).inflate_strm.total_out as libc::c_int;
        (*vpninfo).inflate_adler32 =
            adler32((*vpninfo).inflate_adler32 as uLong,
                    (*new).data.as_mut_ptr(), (*new).len as uInt) as uint32_t;
        pkt_sum =
            load_be32(buf.offset(len as isize).offset(-4) as
                          *const libc::c_void);
        if (*vpninfo).inflate_adler32 != pkt_sum {
            (*vpninfo).quit_reason =
                b"Compression (inflate) adler32 failure\x00" as *const u8 as
                    *const libc::c_char
        }
    } else if compr_type == 1i32 << 1i32 {
        comprname = b"LZS\x00" as *const u8 as *const libc::c_char;
        (*new).len =
            lzs_decompress((*new).data.as_mut_ptr(), receive_mtu, buf, len);
        if (*new).len < 0i32 {
            len = (*new).len;
            if len == 0i32 { len = -22i32 }
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"LZS decompression failed: %s\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        strerror(-len));
            }
            free(new as *mut libc::c_void);
            return len
        }
    } else if compr_type == 1i32 << 2i32 {
        comprname = b"LZ4\x00" as *const u8 as *const libc::c_char;
        (*new).len =
            LZ4_decompress_safe(buf as *mut libc::c_void as
                                    *const libc::c_char,
                                (*new).data.as_mut_ptr() as *mut libc::c_void
                                    as *mut libc::c_char, len, receive_mtu);
        if (*new).len <= 0i32 {
            len = (*new).len;
            if len == 0i32 { len = -22i32 }
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"LZ4 decompression failed\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
            }
            free(new as *mut libc::c_void);
            return len
        }
    } else {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Unknown compression type %d\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    compr_type);
        }
        free(new as *mut libc::c_void);
        return -22i32
    }
    if (*vpninfo).verbose >= 3i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                3i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Received %s compressed data packet of %d bytes (was %d)\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                comprname,
                                                                (*new).len,
                                                                len);
    }
    queue_packet(&mut (*vpninfo).incoming_queue, new);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn compress_packet(mut vpninfo: *mut openconnect_info,
                                         mut compr_type: libc::c_int,
                                         mut this: *mut pkt) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if compr_type == 1i32 << 0i32 {
        (*vpninfo).deflate_strm.next_in = (*this).data.as_mut_ptr();
        (*vpninfo).deflate_strm.avail_in = (*this).len as uInt;
        (*vpninfo).deflate_strm.next_out =
            (*(*vpninfo).deflate_pkt).data.as_mut_ptr() as *mut libc::c_void
                as *mut Bytef;
        (*vpninfo).deflate_strm.avail_out =
            ((*vpninfo).deflate_pkt_size - 4i32) as uInt;
        (*vpninfo).deflate_strm.total_out = 0i32 as uLong;
        ret = deflate(&mut (*vpninfo).deflate_strm, 2i32);
        if ret != 0 {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"deflate failed %d\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        ret);
            }
            /* Things are going to go horribly wrong if we try to do any
			   more compression. Give up entirely. */
            (*vpninfo).cstp_compr = 0i32;
            return -5i32
        }
        /* Add ongoing adler32 to tail of compressed packet */
        (*vpninfo).deflate_adler32 =
            adler32((*vpninfo).deflate_adler32 as uLong,
                    (*this).data.as_mut_ptr(), (*this).len as uInt) as
                uint32_t;
        store_be32(&mut *(*(*vpninfo).deflate_pkt).data.as_mut_ptr().offset((*vpninfo).deflate_strm.total_out
                                                                                as
                                                                                isize)
                       as *mut libc::c_uchar as *mut libc::c_void,
                   (*vpninfo).deflate_adler32);
        (*(*vpninfo).deflate_pkt).len =
            (*vpninfo).deflate_strm.total_out.wrapping_add(4i32 as
                                                               libc::c_ulong)
                as libc::c_int;
        return 0i32
    } else if compr_type == 1i32 << 1i32 {
        if (*this).len < 40i32 { return -27i32 }
        ret =
            lzs_compress((*(*vpninfo).deflate_pkt).data.as_mut_ptr(),
                         (*this).len, (*this).data.as_mut_ptr(), (*this).len);
        if ret < 0i32 { return ret }
        (*(*vpninfo).deflate_pkt).len = ret;
        return 0i32
    } else if compr_type == 1i32 << 2i32 {
        if (*this).len < 40i32 { return -27i32 }
        ret =
            LZ4_compress_default((*this).data.as_mut_ptr() as
                                     *mut libc::c_void as *const libc::c_char,
                                 (*(*vpninfo).deflate_pkt).data.as_mut_ptr()
                                     as *mut libc::c_void as
                                     *mut libc::c_char, (*this).len,
                                 (*this).len);
        if ret <= 0i32 { if ret == 0i32 { ret = -27i32 } return ret }
        (*(*vpninfo).deflate_pkt).len = ret;
        return 0i32
    } else { return -22i32 };
}
#[no_mangle]
pub unsafe extern "C" fn cstp_mainloop(mut vpninfo: *mut openconnect_info,
                                       mut timeout: *mut libc::c_int,
                                       mut readable: libc::c_int)
 -> libc::c_int {
    let mut receive_mtu: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut payload_len: libc::c_int = 0;
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut work_done: libc::c_int = 0i32;
    if (*vpninfo).ssl_fd == -1i32 {
        current_block = 6733435881772574911;
    } else { current_block = 14916268686031723178; }
    'c_30856:
        loop  {
            match current_block {
                6733435881772574911 => {
                    ret = cstp_reconnect(vpninfo);
                    if ret != 0 {
                        if (*vpninfo).verbose >= 0i32 {
                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                    0i32,
                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char,
                                                                                                     b"Reconnect failed\n\x00"
                                                                                                         as
                                                                                                         *const u8
                                                                                                         as
                                                                                                         *const libc::c_char));
                        }
                        (*vpninfo).quit_reason =
                            b"CSTP reconnect failed\x00" as *const u8 as
                                *const libc::c_char;
                        return ret
                    }
                    current_block = 13004291251796920360;
                    break ;
                }
                _ =>
                /* FIXME: The poll() handling here is fairly simplistic. Actually,
	   if the SSL connection stalls it could return a WANT_WRITE error
	   on _either_ of the SSL_read() or SSL_write() calls. In that case,
	   we should probably remove POLLIN from the events we're looking for,
	   and add POLLOUT. As it is, though, it'll just chew CPU time in that
	   fairly unlikely situation, until the write backlog clears. */
                {
                    if readable != 0 {
                        /* Some servers send us packets that are larger than
		   negotiated MTU. We reserve some extra space to
		   handle that */
                        receive_mtu =
                            if 16384i32 >
                                   if (*vpninfo).deflate_pkt_size != 0 {
                                       (*vpninfo).deflate_pkt_size
                                   } else { (*vpninfo).ip_info.mtu } {
                                16384i32
                            } else if (*vpninfo).deflate_pkt_size != 0 {
                                (*vpninfo).deflate_pkt_size
                            } else { (*vpninfo).ip_info.mtu };
                        len = 0;
                        payload_len = 0;
                        if (*vpninfo).cstp_pkt.is_null() {
                            (*vpninfo).cstp_pkt =
                                malloc((::std::mem::size_of::<pkt>() as
                                            libc::c_ulong).wrapping_add(receive_mtu
                                                                            as
                                                                            libc::c_ulong))
                                    as *mut pkt;
                            if (*vpninfo).cstp_pkt.is_null() {
                                if (*vpninfo).verbose >= 0i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            0i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"Allocation failed\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                current_block = 7297078374430259003;
                            } else { current_block = 1856101646708284338; }
                        } else { current_block = 1856101646708284338; }
                        match current_block {
                            7297078374430259003 => { }
                            _ => {
                                len =
                                    ssl_nonblock_read(vpninfo,
                                                      (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr.as_mut_ptr()
                                                          as
                                                          *mut libc::c_void,
                                                      receive_mtu + 8i32);
                                if !(len == 0) {
                                    if len < 0i32 {
                                        current_block = 6733435881772574911;
                                        continue ;
                                    }
                                    if len < 8i32 {
                                        if (*vpninfo).verbose >= 0i32 {
                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    0i32,
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Short packet received (%d bytes)\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char),
                                                                                                    len);
                                        }
                                        (*vpninfo).quit_reason =
                                            b"Short packet received\x00" as
                                                *const u8 as
                                                *const libc::c_char;
                                        return 1i32
                                    }
                                    if (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[0]
                                           as libc::c_int != 'S' as i32 ||
                                           (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[1]
                                               as libc::c_int != 'T' as i32 ||
                                           (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[2]
                                               as libc::c_int != 'F' as i32 ||
                                           (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[3]
                                               as libc::c_int != 1i32 ||
                                           (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[7]
                                               as libc::c_int != 0 {
                                        current_block = 994217605032671458;
                                        break ;
                                    }
                                    payload_len =
                                        load_be16((*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr.as_mut_ptr().offset(4)
                                                      as *const libc::c_void)
                                            as libc::c_int;
                                    if len != 8i32 + payload_len {
                                        if (*vpninfo).verbose >= 0i32 {
                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    0i32,
                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     b"Unexpected packet length. SSL_read returned %d but packet is\n\x00"
                                                                                                                         as
                                                                                                                         *const u8
                                                                                                                         as
                                                                                                                         *const libc::c_char),
                                                                                                    len);
                                        }
                                        if (*vpninfo).verbose >= 0i32 {
                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                    0i32,
                                                                                                    b"%02x %02x %02x %02x %02x %02x %02x %02x\n\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[0]
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[1]
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[2]
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[3]
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[4]
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[5]
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[6]
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[7]
                                                                                                        as
                                                                                                        libc::c_int);
                                        }
                                        current_block = 14916268686031723178;
                                        continue ;
                                    } else {
                                        (*vpninfo).ssl_times.last_rx =
                                            time(0 as *mut time_t);
                                        match (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[6]
                                                  as libc::c_int {
                                            3 => {
                                                if (*vpninfo).verbose >= 2i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            2i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Got CSTP DPD request\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char));
                                                }
                                                (*vpninfo).owe_ssl_dpd_response
                                                    = 1i32;
                                                current_block =
                                                    14916268686031723178;
                                                continue ;
                                            }
                                            4 => {
                                                if (*vpninfo).verbose >= 2i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            2i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Got CSTP DPD response\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char));
                                                }
                                                current_block =
                                                    14916268686031723178;
                                                continue ;
                                            }
                                            7 => {
                                                if (*vpninfo).verbose >= 2i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            2i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Got CSTP Keepalive\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char));
                                                }
                                                current_block =
                                                    14916268686031723178;
                                                continue ;
                                            }
                                            0 => {
                                                if (*vpninfo).verbose >= 3i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            3i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Received uncompressed data packet of %d bytes\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char),
                                                                                                            payload_len);
                                                }
                                                (*(*vpninfo).cstp_pkt).len =
                                                    payload_len;
                                                queue_packet(&mut (*vpninfo).incoming_queue,
                                                             (*vpninfo).cstp_pkt);
                                                (*vpninfo).cstp_pkt =
                                                    0 as *mut pkt;
                                                work_done = 1i32;
                                                current_block =
                                                    14916268686031723178;
                                                continue ;
                                            }
                                            5 => {
                                                let mut i: libc::c_int = 0;
                                                if payload_len >= 2i32 {
                                                    i = 1i32;
                                                    while i < payload_len {
                                                        if isprint(*(*(*vpninfo).cstp_pkt).data.as_mut_ptr().offset(i
                                                                                                                        as
                                                                                                                        isize)
                                                                       as
                                                                       libc::c_int)
                                                               == 0 {
                                                            *(*(*vpninfo).cstp_pkt).data.as_mut_ptr().offset(i
                                                                                                                 as
                                                                                                                 isize)
                                                                =
                                                                '.' as i32 as
                                                                    libc::c_uchar
                                                        }
                                                        i += 1
                                                    }
                                                    *(*(*vpninfo).cstp_pkt).data.as_mut_ptr().offset(payload_len
                                                                                                         as
                                                                                                         isize)
                                                        =
                                                        0i32 as libc::c_uchar;
                                                    if (*vpninfo).verbose >=
                                                           0i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                0i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Received server disconnect: %02x \'%s\'\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char),
                                                                                                                *(*(*vpninfo).cstp_pkt).data.as_mut_ptr().offset(0)
                                                                                                                    as
                                                                                                                    libc::c_int,
                                                                                                                (*(*vpninfo).cstp_pkt).data.as_mut_ptr().offset(1));
                                                    }
                                                } else if (*vpninfo).verbose
                                                              >= 0i32 {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            0i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"Received server disconnect\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char));
                                                }
                                                (*vpninfo).quit_reason =
                                                    b"Server request\x00" as
                                                        *const u8 as
                                                        *const libc::c_char;
                                                return -32i32
                                            }
                                            8 => {
                                                if (*vpninfo).cstp_compr == 0
                                                   {
                                                    if (*vpninfo).verbose >=
                                                           0i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                0i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Compressed packet received in !deflate mode\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char));
                                                    }
                                                    current_block =
                                                        994217605032671458;
                                                    break ;
                                                } else {
                                                    decompress_and_queue_packet(vpninfo,
                                                                                (*vpninfo).cstp_compr,
                                                                                (*(*vpninfo).cstp_pkt).data.as_mut_ptr(),
                                                                                payload_len);
                                                    work_done = 1i32;
                                                    current_block =
                                                        14916268686031723178;
                                                    continue ;
                                                }
                                            }
                                            9 => {
                                                if (*vpninfo).verbose >= 0i32
                                                   {
                                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                            0i32,
                                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char,
                                                                                                                             b"received server terminate packet\n\x00"
                                                                                                                                 as
                                                                                                                                 *const u8
                                                                                                                                 as
                                                                                                                                 *const libc::c_char));
                                                }
                                                (*vpninfo).quit_reason =
                                                    b"Server request\x00" as
                                                        *const u8 as
                                                        *const libc::c_char;
                                                return -32i32
                                            }
                                            _ => {
                                                current_block =
                                                    994217605032671458;
                                                break ;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    /* If SSL_write() fails we are expected to try again. With exactly
	   the same data, at exactly the same location. So we keep the
	   packet we had before.... */
                    if !(*vpninfo).current_ssl_pkt.is_null() {
                        current_block = 13343580537917585478;
                    } else { current_block = 2222055338596505704; }
                    loop  {
                        match current_block {
                            13343580537917585478 => {
                                (*vpninfo).ssl_times.last_tx =
                                    time(0 as *mut time_t);
                                let mut __fd: libc::c_int = (*vpninfo).ssl_fd;
                                (*vpninfo)._select_wfds.fds_bits[(__fd as
                                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<__int32_t>()
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_mul(8i32
                                                                                                                                       as
                                                                                                                                       libc::c_ulong))
                                                                     as usize]
                                    &=
                                    !(((1i32 as libc::c_ulong) <<
                                           (__fd as
                                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<__int32_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(8i32
                                                                                                                 as
                                                                                                                 libc::c_ulong)))
                                          as __int32_t);
                                ret =
                                    ssl_nonblock_write(vpninfo,
                                                       (*(*vpninfo).current_ssl_pkt).c2rust_unnamed.cstp.hdr.as_mut_ptr()
                                                           as
                                                           *mut libc::c_void,
                                                       (*(*vpninfo).current_ssl_pkt).len
                                                           + 8i32);
                                if ret < 0i32 {
                                    current_block = 6733435881772574911;
                                    continue 'c_30856 ;
                                }
                                if ret == 0 {
                                    /* -EAGAIN: ssl_nonblock_write() will have added the SSL
			   fd to ->select_wfds if appropriate, so we can just
			   return and wait. Unless it's been stalled for so long
			   that DPD kicks in and we kill the connection. */
                                    match ka_stalled_action(&mut (*vpninfo).ssl_times,
                                                            timeout) {
                                        2 => { break ; }
                                        4 => {
                                            current_block =
                                                12676590665067494733;
                                        }
                                        0 => { return work_done }
                                        _ => {
                                            current_block =
                                                14953815020842398287;
                                        }
                                    }
                                } else {
                                    current_block = 14953815020842398287;
                                }
                                match current_block {
                                    12676590665067494733 => { }
                                    _ =>
                                    /* This should never happen */
                                    {
                                        if ret !=
                                               (*(*vpninfo).current_ssl_pkt).len
                                                   + 8i32 {
                                            if (*vpninfo).verbose >= 0i32 {
                                                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                        0i32,
                                                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char,
                                                                                                                         b"SSL wrote too few bytes! Asked for %d, sent %d\n\x00"
                                                                                                                             as
                                                                                                                             *const u8
                                                                                                                             as
                                                                                                                             *const libc::c_char),
                                                                                                        (*(*vpninfo).current_ssl_pkt).len
                                                                                                            +
                                                                                                            8i32,
                                                                                                        ret);
                                            }
                                            (*vpninfo).quit_reason =
                                                b"Internal error\x00" as
                                                    *const u8 as
                                                    *const libc::c_char;
                                            return 1i32
                                        }
                                        /* Don't free the 'special' packets */
                                        if (*vpninfo).current_ssl_pkt ==
                                               (*vpninfo).deflate_pkt {
                                            free((*vpninfo).pending_deflated_pkt
                                                     as *mut libc::c_void);
                                            (*vpninfo).pending_deflated_pkt =
                                                0 as *mut pkt
                                        } else if (*vpninfo).current_ssl_pkt
                                                      !=
                                                      &dpd_pkt as *const pkt
                                                          as *mut pkt &&
                                                      (*vpninfo).current_ssl_pkt
                                                          !=
                                                          &dpd_resp_pkt as
                                                              *const pkt as
                                                              *mut pkt &&
                                                      (*vpninfo).current_ssl_pkt
                                                          !=
                                                          &keepalive_pkt as
                                                              *const pkt as
                                                              *mut pkt {
                                            free((*vpninfo).current_ssl_pkt as
                                                     *mut libc::c_void);
                                        }
                                        (*vpninfo).current_ssl_pkt =
                                            0 as *mut pkt;
                                        current_block = 2222055338596505704;
                                        continue ;
                                    }
                                }
                            }
                            _ => {
                                if (*vpninfo).owe_ssl_dpd_response != 0 {
                                    (*vpninfo).owe_ssl_dpd_response = 0i32;
                                    (*vpninfo).current_ssl_pkt =
                                        &dpd_resp_pkt as *const pkt as
                                            *mut pkt;
                                    current_block = 13343580537917585478;
                                    continue ;
                                } else {
                                    match keepalive_action(&mut (*vpninfo).ssl_times,
                                                           timeout) {
                                        4 => {
                                            current_block =
                                                12676590665067494733;
                                        }
                                        2 => { break ; }
                                        1 => {
                                            current_block =
                                                964430785027261791;
                                            match current_block {
                                                964430785027261791 => {
                                                    if (*vpninfo).verbose >=
                                                           2i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                2i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Send CSTP DPD\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char));
                                                    }
                                                    (*vpninfo).current_ssl_pkt
                                                        =
                                                        &dpd_pkt as *const pkt
                                                            as *mut pkt;
                                                    current_block =
                                                        13343580537917585478;
                                                    continue ;
                                                }
                                                _ =>
                                                /* No need to send an explicit keepalive
		   if we have real data to send */
                                                {
                                                    if !((*vpninfo).dtls_state
                                                             != 5i32 &&
                                                             !(*vpninfo).outgoing_queue.head.is_null())
                                                       {
                                                        if (*vpninfo).verbose
                                                               >= 2i32 {
                                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                    2i32,
                                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char,
                                                                                                                                     b"Send CSTP Keepalive\n\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char));
                                                        }
                                                        (*vpninfo).current_ssl_pkt
                                                            =
                                                            &keepalive_pkt as
                                                                *const pkt as
                                                                *mut pkt;
                                                        current_block =
                                                            13343580537917585478;
                                                        continue ;
                                                    }
                                                }
                                            }
                                            current_block =
                                                6538935128550440286;
                                        }
                                        3 => {
                                            current_block =
                                                16114569038092921525;
                                            match current_block {
                                                964430785027261791 => {
                                                    if (*vpninfo).verbose >=
                                                           2i32 {
                                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                2i32,
                                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char,
                                                                                                                                 b"Send CSTP DPD\n\x00"
                                                                                                                                     as
                                                                                                                                     *const u8
                                                                                                                                     as
                                                                                                                                     *const libc::c_char));
                                                    }
                                                    (*vpninfo).current_ssl_pkt
                                                        =
                                                        &dpd_pkt as *const pkt
                                                            as *mut pkt;
                                                    current_block =
                                                        13343580537917585478;
                                                    continue ;
                                                }
                                                _ => {
                                                    if !((*vpninfo).dtls_state
                                                             != 5i32 &&
                                                             !(*vpninfo).outgoing_queue.head.is_null())
                                                       {
                                                        if (*vpninfo).verbose
                                                               >= 2i32 {
                                                            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                                    2i32,
                                                                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char,
                                                                                                                                     b"Send CSTP Keepalive\n\x00"
                                                                                                                                         as
                                                                                                                                         *const u8
                                                                                                                                         as
                                                                                                                                         *const libc::c_char));
                                                        }
                                                        (*vpninfo).current_ssl_pkt
                                                            =
                                                            &keepalive_pkt as
                                                                *const pkt as
                                                                *mut pkt;
                                                        current_block =
                                                            13343580537917585478;
                                                        continue ;
                                                    }
                                                }
                                            }
                                            current_block =
                                                6538935128550440286;
                                        }
                                        0 | _ => {
                                            current_block =
                                                6538935128550440286;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            12676590665067494733 => {
                                /* Not that this will ever happen; we don't even process
		   the setting when we're asked for it. */
                                if (*vpninfo).verbose >= 1i32 {
                                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                            1i32,
                                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char,
                                                                                                             b"CSTP rekey due\n\x00"
                                                                                                                 as
                                                                                                                 *const u8
                                                                                                                 as
                                                                                                                 *const libc::c_char));
                                }
                                if (*vpninfo).ssl_times.rekey_method == 1i32 {
                                    current_block = 6733435881772574911;
                                    continue 'c_30856 ;
                                }
                                if (*vpninfo).ssl_times.rekey_method == 2i32 {
                                    ret =
                                        cstp_handshake(vpninfo,
                                                       0i32 as libc::c_uint);
                                    if !(ret != 0) {
                                        current_block = 13004291251796920360;
                                        break 'c_30856 ;
                                    }
                                    /* if we failed rehandshake try establishing a new-tunnel instead of failing */
                                    if (*vpninfo).verbose >= 0i32 {
                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                0i32,
                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char,
                                                                                                                 b"Rehandshake failed; attempting new-tunnel\n\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char));
                                    }
                                    current_block = 6733435881772574911;
                                    continue 'c_30856 ;
                                }
                            }
                            _ => { }
                        }
                        /* Service outgoing packet queue, if no DTLS */
                        if (*vpninfo).dtls_state != 5i32 &&
                               {
                                   (*vpninfo).current_ssl_pkt =
                                       dequeue_packet(&mut (*vpninfo).outgoing_queue);
                                   !(*vpninfo).current_ssl_pkt.is_null()
                               } {
                            let mut this: *mut pkt =
                                (*vpninfo).current_ssl_pkt;
                            let mut current_block_159: u64;
                            if (*vpninfo).cstp_compr != 0 {
                                ret =
                                    compress_packet(vpninfo,
                                                    (*vpninfo).cstp_compr,
                                                    this);
                                if ret < 0i32 {
                                    current_block_159 = 2435423126632341978;
                                } else {
                                    store_be16((*(*vpninfo).deflate_pkt).c2rust_unnamed.cstp.hdr.as_mut_ptr().offset(4)
                                                   as *mut libc::c_void,
                                               (*(*vpninfo).deflate_pkt).len
                                                   as uint16_t);
                                    /* DTLS compression may have screwed with this */
                                    (*(*vpninfo).deflate_pkt).c2rust_unnamed.cstp.hdr[7]
                                        = 0i32 as libc::c_uchar;
                                    if (*vpninfo).verbose >= 3i32 {
                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                3i32,
                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char,
                                                                                                                 b"Sending compressed data packet of %d bytes (was %d)\n\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char),
                                                                                                (*(*vpninfo).deflate_pkt).len,
                                                                                                (*this).len);
                                    }
                                    (*vpninfo).pending_deflated_pkt = this;
                                    (*vpninfo).current_ssl_pkt =
                                        (*vpninfo).deflate_pkt;
                                    current_block_159 = 18340277188286182087;
                                }
                            } else {
                                current_block_159 = 2435423126632341978;
                            }
                            match current_block_159 {
                                2435423126632341978 => {
                                    memcpy((*this).c2rust_unnamed.cstp.hdr.as_mut_ptr()
                                               as *mut libc::c_void,
                                           data_hdr.as_ptr() as
                                               *const libc::c_void,
                                           8i32 as libc::c_ulong);
                                    store_be16((*this).c2rust_unnamed.cstp.hdr.as_mut_ptr().offset(4)
                                                   as *mut libc::c_void,
                                               (*this).len as uint16_t);
                                    if (*vpninfo).verbose >= 3i32 {
                                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                                3i32,
                                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char,
                                                                                                                 b"Sending uncompressed data packet of %d bytes\n\x00"
                                                                                                                     as
                                                                                                                     *const u8
                                                                                                                     as
                                                                                                                     *const libc::c_char),
                                                                                                (*this).len);
                                    }
                                    (*vpninfo).current_ssl_pkt = this
                                }
                                _ => { }
                            }
                            current_block = 13343580537917585478;
                        } else {
                            /* Work is not done if we just got rid of packets off the queue */
                            return work_done
                        }
                    }
                    if (*vpninfo).verbose >= 0i32 {
                        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                                0i32,
                                                                                libintl_dgettext(b"openconnect\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 b"CSTP Dead Peer Detection detected dead peer!\n\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char));
                    }
                    current_block = 6733435881772574911;
                }
            }
        }
    match current_block {
        13004291251796920360 => {
            /* succeeded, let's rekey DTLS, if it is not rekeying
		 * itself. */
            if (*vpninfo).dtls_state > 3i32 &&
                   (*vpninfo).dtls_times.rekey_method == 0i32 {
                (*vpninfo).dtls_need_reconnect = 1i32
            }
            return 1i32
        }
        _ => {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unknown packet %02x %02x %02x %02x %02x %02x %02x %02x\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[0]
                                                                            as
                                                                            libc::c_int,
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[1]
                                                                            as
                                                                            libc::c_int,
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[2]
                                                                            as
                                                                            libc::c_int,
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[3]
                                                                            as
                                                                            libc::c_int,
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[4]
                                                                            as
                                                                            libc::c_int,
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[5]
                                                                            as
                                                                            libc::c_int,
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[6]
                                                                            as
                                                                            libc::c_int,
                                                                        (*(*vpninfo).cstp_pkt).c2rust_unnamed.cstp.hdr[7]
                                                                            as
                                                                            libc::c_int);
            }
            (*vpninfo).quit_reason =
                b"Unknown packet received\x00" as *const u8 as
                    *const libc::c_char;
            return 1i32
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cstp_bye(mut vpninfo: *mut openconnect_info,
                                  mut reason: *const libc::c_char)
 -> libc::c_int {
    let mut bye_pkt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut reason_len: libc::c_int = 0;
    /* already lost connection? */
    if (*vpninfo).https_ssl.is_null() { return 0i32 }
    reason_len = strlen(reason) as libc::c_int;
    bye_pkt =
        malloc((reason_len + 9i32) as libc::c_ulong) as *mut libc::c_uchar;
    if bye_pkt.is_null() { return -12i32 }
    memcpy(bye_pkt as *mut libc::c_void,
           data_hdr.as_ptr() as *const libc::c_void, 8i32 as libc::c_ulong);
    memcpy(bye_pkt.offset(9) as *mut libc::c_void,
           reason as *const libc::c_void, reason_len as libc::c_ulong);
    store_be16(bye_pkt.offset(4) as *mut libc::c_void,
               (reason_len + 1i32) as uint16_t);
    *bye_pkt.offset(6) = 5i32 as libc::c_uchar;
    *bye_pkt.offset(8) = 0xb0i32 as libc::c_uchar;
    if (*vpninfo).verbose >= 1i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                1i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Send BYE packet: %s\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                reason);
    }
    ssl_nonblock_write(vpninfo, bye_pkt as *mut libc::c_void,
                       reason_len + 9i32);
    free(bye_pkt as *mut libc::c_void);
    return 0i32;
}
/* Key material for DTLS-PSK */
/* Packet types */
/* Uncompressed data */
/* Dead Peer Detection */
/* DPD response */
/* Client disconnection notice */
/* Keepalive */
/* Compressed data */
/* Server kick */
/* Encryption and HMAC algorithms (matching Juniper/Pulse binary encoding) */
/* SHA256 */
/* Including the next-header field */
/* ***************************************************************************/
/* Oh Solaris how we hate thee! */
/* For systems that don't support O_CLOEXEC, just don't bother.
   We don't keep files open for long anyway. */
/* I always coded as if it worked like this. Now it does. */
/* ***************************************************************************/
/* iconv.c */
/* script.c */
/* tun.c / tun-win32.c */
/* {gnutls,openssl}-dtls.c */
/* dtls.c */
/* cstp.c */
#[no_mangle]
pub unsafe extern "C" fn cstp_common_headers(mut vpninfo:
                                                 *mut openconnect_info,
                                             mut buf: *mut oc_text_buf) {
    http_common_headers(vpninfo, buf);
    buf_append(buf,
               b"Accept: */*\r\n\x00" as *const u8 as *const libc::c_char);
    buf_append(buf,
               b"Accept-Encoding: identity\r\n\x00" as *const u8 as
                   *const libc::c_char);
    buf_append(buf,
               b"X-Transcend-Version: 1\r\n\x00" as *const u8 as
                   *const libc::c_char);
    if (*vpninfo).xmlpost != 0 {
        buf_append(buf,
                   b"X-Aggregate-Auth: 1\r\n\x00" as *const u8 as
                       *const libc::c_char);
        buf_append(buf,
                   b"X-AnyConnect-Platform: %s\r\n\x00" as *const u8 as
                       *const libc::c_char, (*vpninfo).platname);
    }
    if (*vpninfo).try_http_auth != 0 {
        buf_append(buf,
                   b"X-Support-HTTP-Auth: true\r\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    append_mobile_headers(vpninfo, buf);
}
