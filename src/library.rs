#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type stoken_ctx;
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
    pub type internal_state;
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
    /* Failed */
    /* Server has not offered it */
    /* Server has offered it, we have not tried it */
    /* Individual auth types may use 2 onwards for their own state */
    /* In-progress attempt */
    /* This does the full authentication, calling back as appropriate */
    /* Establish the TCP connection (and obtain configuration) */
    /* Add headers common to each HTTP request */
    /* Set up the UDP (DTLS) connection. Doesn't actually *start* it. */
    /* This will actually complete the UDP connection setup/handshake on the wire,
	   as well as transporting packets */
    /* Close the connection but leave the session setup so it restarts */
    /* Close and destroy the (UDP) session */
    /* Send probe packets to start or maintain the (UDP) session */
    /* Catch probe packet confirming the (UDP) session */
    /* packet + header */
    /* DTLS header */
    /* biggest supported MAC (SHA1) */
    /* biggest supported IV (AES-256) */
    /* max padding */
    /* Stored network-endian */
    /* Encryption key */
    /* HMAC key */
    pub type oc_pcsc_ctx;
    pub type gss_ctx_id_struct;
    pub type gss_name_struct;
    pub type pxProxyFactory_;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                   _: libc::c_ulong) -> libc::c_int;
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
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn close(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pipe(_: *mut libc::c_int) -> libc::c_int;
    /* !__DARWIN_UNIX03 */
    /* __DARWIN_UNIX03 */
    #[no_mangle]
    fn unlink(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fcntl(_: libc::c_int, _: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn stoken_destroy(ctx: *mut stoken_ctx);
    #[no_mangle]
    fn xmlFreeNode(cur: xmlNodePtr);
    /* Allocates descriptor for code conversion from encoding `fromcode' to
   encoding `tocode'. */
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char,
                  __fromcode: *const libc::c_char) -> iconv_t;
    /* Frees resources allocated for conversion descriptor `cd'. */
    #[no_mangle]
    fn iconv_close(_cd: iconv_t) -> libc::c_int;
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
    #[no_mangle]
    fn openconnect_bin2base64(prefix: *const libc::c_char,
                              data: *const uint8_t, len: libc::c_uint)
     -> *mut libc::c_char;
    /* Message catalogs for internationalization.
   Copyright (C) 1995-1997, 2000-2016, 2018-2019 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation; either version 2.1 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
    /* The LC_MESSAGES locale category is the category used by the functions
   gettext() and dgettext().  It is specified in POSIX, but not in ANSI C.
   On systems that don't define it, use an arbitrary value instead.
   On Solaris, <locale.h> defines __LOCALE_H (or _LOCALE_H in Solaris 2.5)
   then includes <libintl.h> (i.e. this file!) and then only defines
   LC_MESSAGES.  To avoid a redefinition warning, don't define LC_MESSAGES
   in this case.  */
    /* We define an additional symbol to signal that we use the GNU
   implementation of gettext.  */
    /* Provide information about the supported file formats.  Returns the
   maximum minor revision number supported for a given major revision.  */
    /* Resolve a platform specific conflict on DJGPP.  GNU gettext takes
   precedence over _conio_gettext.  */
    /* Version number: (major<<16) + (minor<<8) + subminor */
    /* We redirect the functions to those prefixed with "libintl_".  This is
   necessary, because some systems define gettext/textdomain/... in the C
   library (namely, Solaris 2.4 and newer, and GNU libc 2.0 and newer).
   If we used the unprefixed names, there would be cases where the
   definition in the C library would override the one in the libintl.so
   shared library.  Recall that on ELF systems, the symbols are looked
   up in the following order:
     1. in the executable,
     2. in the shared libraries specified on the link command line, in order,
     3. in the dependencies of the shared libraries specified on the link
        command line,
     4. in the dlopen()ed shared libraries, in the order in which they were
        dlopen()ed.
   The definition in the C library would override the one in libintl.so if
   either
     * -lc is given on the link command line and -lintl isn't, or
     * -lc is given on the link command line before -lintl, or
     * libintl.so is a dependency of a dlopen()ed shared library but not
       linked to the executable at link time.
   Since Solaris gettext() behaves differently than GNU gettext(), this
   would be unacceptable.

   The redirection happens by default through macros in C, so that &gettext
   is independent of the compilation unit, but through inline functions in
   C++, in order not to interfere with the name mangling of class fields or
   class methods called 'gettext'.  */
    /* The user can define _INTL_REDIRECT_INLINE or _INTL_REDIRECT_MACROS.
   If he doesn't, we choose the method.  A third possible method is
   _INTL_REDIRECT_ASM, supported only by GCC.  */
    /* Auxiliary macros.  */
    /* _INTL_MAY_RETURN_STRING_ARG(n) declares that the given function may return
   its n-th argument literally.  This enables GCC to warn for example about
   printf (gettext ("foo %y")).  */
    /* Look up MSGID in the current default message catalog for the current
   LC_MESSAGES locale.  If not found, returns MSGID itself (the default
   text).  */
    /* Look up MSGID in the DOMAINNAME message catalog for the current
   LC_MESSAGES locale.  */
    #[no_mangle]
    fn libintl_dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn openconnect_bin2hex(prefix: *const libc::c_char, data: *const uint8_t,
                           len: libc::c_uint) -> *mut libc::c_char;
    #[no_mangle]
    fn openconnect_sha1(result: *mut libc::c_uchar, data: *mut libc::c_void,
                        len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn openconnect_get_peer_cert_DER(vpninfo: *mut openconnect_info,
                                     buf: *mut *mut libc::c_uchar)
     -> libc::c_int;
    #[no_mangle]
    fn prepare_stoken(vpninfo: *mut openconnect_info) -> libc::c_int;
    /* OPENSSL_NO_EC */
    /* enough comments already ... see SSL_CTX_set_session_cache_mode(3) */
    /*
 * the maximum length of the buffer given to callbacks containing the
 * resulting identity/psk
 */
    /* Register callbacks to handle custom TLS Extensions for client or server. */
    /* These will only be used when doing non-blocking IO */
    /*
     * protocol version (one of SSL2_VERSION, SSL3_VERSION, TLS1_VERSION,
     * DTLS1_VERSION)
     */
    /* SSL_ST_CONNECT or SSL_ST_ACCEPT */
    /* SSLv3 */
    /*
     * There are 2 BIO's even though they are normally both the same.  This
     * is so data can be read and written to different handlers
     */
    /* used by SSL_read */
    /* used by SSL_write */
    /* used during session-id reuse to concatenate messages */
    /*
     * This holds a variable that indicates what we were doing when a 0 or -1
     * is returned.  This is needed for non-blocking IO so we know what
     * request needs re-doing when in SSL_accept or SSL_connect
     */
    /* true when we are actually in SSL_accept() or SSL_connect() */
    /*
     * Imagine that here's a boolean member "init" that is switched as soon
     * as SSL_set_{accept/connect}_state is called for the first time, so
     * that "state" and "handshake_func" are properly initialized.  But as
     * handshake_func is == 0 until then, we use this test instead of an
     * "init" member.
     */
    /* are we the server side? - mostly used by SSL_clear */
    /*
     * Generate a new session or reuse an old one.
     * NB: For servers, the 'new' session may actually be a previously
     * cached session or even the previous session unless
     * SSL_OP_NO_SESSION_RESUMPTION_ON_RENEGOTIATION is set
     */
    /* don't send shutdown packets */
    /* we have shut things down, 0x01 sent, 0x02 for received */
    /* where we are */
    /* where we are when reading */
    /* buffer used during init */
    /* pointer to handshake message body, set by
                                 * ssl3_get_message() */
    /* amount read/written */
    /* amount read/written */
    /* used internally to point at a raw packet */
    /* SSLv2 variables */
    /* SSLv3 variables */
    /* DTLSv1 variables */
    /* Read as many input bytes as possible (for
                                 * non-blocking reads) */
    /* callback that allows applications to peek at protocol messages */
    /* reusing a previous session */
    /* crypto */
    /*
     * These are the ones being used, the ones in SSL_SESSION are the ones to
     * be 'copied' into these ones
     */
    /* cryptographic state */
    /* used for mac generation */
    /* uncompress */
    /* cryptographic state */
    /* used for mac generation */
    /* compression */
    /* session info */
    /* client cert? */
    /* This is used to hold the server certificate used */
    /* CERT */
    /*
     * the session_id_context is used to ensure sessions are only reused in
     * the appropriate context
     */
    /* This can also be in the session once a session is established */
    /* Default generate session ID callback. */
    /* Used in SSL2 and SSL3 */
    /*
     * 0 don't care about verify failure.
     * 1 fail if verify fails
     */
    /* fail if callback returns 0 */
    /* optional informational callback */
    /* error bytes to be written */
    /* actual code */
    /* OPENSSL_NO_KRB5 */
    /*
     * set this flag to 1 and a sleep(1) is put into all SSL_read() and
     * SSL_write() calls, good for nbio debuging :-)
     */
    /* extra application data */
    /* for server side, keep the list of CA_dn we can use */
    /* protocol behaviour */
    /* API behaviour */
    /* what was passed, used for SSLv3/TLS rollback check */
    /* TLS extension debug callback */
    /*-
     * no further mod of servername
     * 0 : call the servername extension callback.
     * 1 : prepare 2, allow last ack just after in server callback.
     * 2 : don't call servername callback, no ack in server hello
     */
    /* certificate status request info */
    /* Status type or -1 if no status type */
    /* Expect OCSP CertificateStatus message */
    /* OCSP status request only */
    /* OCSP response received or to be sent */
    /* RFC4507 session ticket expected to be received or sent */
    /* our list */
    /* our list */
    /* OPENSSL_NO_EC */
    /*
     * draft-rescorla-tls-opaque-prf-input-00.txt information to be used for
     * handshakes
     */
    /* TLS Session Ticket extension override */
    /* TLS Session Ticket extension callback */
    /* TLS pre-shared secret session resumption */
    /* initial ctx, used to store sessions */
    /*
     * Next protocol negotiation. For the client, this is the protocol that
     * we sent in NextProtocol and is set when handling ServerHello
     * extensions. For a server, this is the client's selected_protocol from
     * NextProtocol and is set when handling the NextProtocol message, before
     * the Finished message.
     */
    /* What we'll do */
    /* What's been chosen */
    /*-
         * Is use of the Heartbeat extension negotiated?
         * 0: disabled
         * 1: enabled
         * 2: enabled, but not allowed to send Requests
         */
    /* Indicates if a HeartbeatRequest is in flight */
    /* HeartbeatRequest sequence number */
    /* OPENSSL_NO_TLSEXT */
    /*-
     * 1 if we are renegotiating.
     * 2 if we are a server and are inside a handshake
     * (i.e. not just sending a HelloRequest)
     */
    /* ctx for SRP authentication */
    /*
     * For a client, this contains the list of supported protocols in wire
     * format.
     */
    /* OPENSSL_NO_TLSEXT */
    /* compatibility */
    /*
 * The following are the possible values for ssl->state are are used to
 * indicate where we are up to in the SSL connection establishment. The
 * macros that follow are about the only things you should need to use and
 * even then, only when using non-blocking IO. It can also be useful to work
 * out where you were when the connection failed
 */
    /* used in callback */
    /* Is the SSL_connection established? */
    /*
 * The following 2 states are kept in ssl->rstate when reads fail, you should
 * not need these
 */
    /*-
 * Obtain latest Finished message
 *   -- that we sent (SSL_get_finished)
 *   -- that we expected from peer (SSL_get_peer_finished).
 * Returns length (0 == no Finished so far), copies up to 'count' bytes.
 */
    /*
 * use either SSL_VERIFY_NONE or SSL_VERIFY_PEER, the last 2 options are
 * 'ored' with SSL_VERIFY_PEER if they are desired
 */
    /* this is for backward compatibility */
    /* NEW_SSLEAY */
    /* More backward compatibility */
    /* offset to get SSL_R_... value
                                              * from SSL_AD_... */
    /* These alert types are for SSLv3 and TLSv1 */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* Not for TLS */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* fatal */
    /* look at error stack/return
                                           * value/errno */
    /* only applies to datagram connections */
    /* Stats */
    /* see tls1.h for macros based on these */
    /* OPENSSL_NO_TLSEXT */
    #[no_mangle]
    fn SSL_get_current_cipher(s: *const SSL) -> *const SSL_CIPHER;
    #[no_mangle]
    fn SSL_CIPHER_get_name(c: *const SSL_CIPHER) -> *const libc::c_char;
    #[no_mangle]
    fn buf_append_utf16le(buf: *mut oc_text_buf, utf8: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn set_yubikey_mode(vpninfo: *mut openconnect_info,
                        token_str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn set_hotp_mode(vpninfo: *mut openconnect_info,
                     token_str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn set_totp_mode(vpninfo: *mut openconnect_info,
                     token_str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn set_libstoken_mode(vpninfo: *mut openconnect_info,
                          token_str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn openconnect_clear_cookies(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn openconnect_close_https(vpninfo: *mut openconnect_info,
                               final_0: libc::c_int);
    #[no_mangle]
    fn internal_parse_url(url: *const libc::c_char,
                          res_proto: *mut *mut libc::c_char,
                          res_host: *mut *mut libc::c_char,
                          res_port: *mut libc::c_int,
                          res_path: *mut *mut libc::c_char,
                          default_port: libc::c_int) -> libc::c_int;
    /* This is private for now since we haven't yet worked out what the API will be */
    /* version.c */
    #[no_mangle]
    static mut openconnect_version_str: *const libc::c_char;
    #[no_mangle]
    fn openconnect_setup_tun_fd(vpninfo: *mut openconnect_info,
                                tun_fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn script_config_tun(vpninfo: *mut openconnect_info,
                         reason: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn script_setenv(vpninfo: *mut openconnect_info, opt: *const libc::c_char,
                     val: *const libc::c_char, trunc: libc::c_int,
                     append: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn openconnect_utf8_to_legacy(vpninfo: *mut openconnect_info,
                                  utf8: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn os_setup_tun(vpninfo: *mut openconnect_info) -> intptr_t;
    #[no_mangle]
    fn prepare_script_env(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn oncp_esp_catch_probe(vpninfo: *mut openconnect_info, pkt: *mut pkt)
     -> libc::c_int;
    #[no_mangle]
    fn oncp_esp_send_probes(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn esp_shutdown(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn esp_close(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn esp_mainloop(vpninfo: *mut openconnect_info, timeout: *mut libc::c_int,
                    readable: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn esp_setup(vpninfo: *mut openconnect_info,
                 dtls_attempt_period: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pulse_obtain_cookie(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn http_common_headers(vpninfo: *mut openconnect_info,
                           buf: *mut oc_text_buf);
    #[no_mangle]
    fn pulse_mainloop(vpninfo: *mut openconnect_info,
                      timeout: *mut libc::c_int, readable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn pulse_connect(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn pulse_bye(vpninfo: *mut openconnect_info, reason: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn gpst_esp_catch_probe(vpninfo: *mut openconnect_info, pkt: *mut pkt)
     -> libc::c_int;
    #[no_mangle]
    fn gpst_esp_send_probes(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn gpst_obtain_cookie(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn gpst_common_headers(vpninfo: *mut openconnect_info,
                           buf: *mut oc_text_buf);
    #[no_mangle]
    fn gpst_mainloop(vpninfo: *mut openconnect_info,
                     timeout: *mut libc::c_int, readable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn gpst_setup(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn gpst_bye(vpninfo: *mut openconnect_info, reason: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn oncp_esp_close(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn oncp_obtain_cookie(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn oncp_common_headers(vpninfo: *mut openconnect_info,
                           buf: *mut oc_text_buf);
    #[no_mangle]
    fn oncp_mainloop(vpninfo: *mut openconnect_info,
                     timeout: *mut libc::c_int, readable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn oncp_connect(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn oncp_bye(vpninfo: *mut openconnect_info, reason: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn dtls_shutdown(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn dtls_close(vpninfo: *mut openconnect_info);
    #[no_mangle]
    fn dtls_mainloop(vpninfo: *mut openconnect_info,
                     timeout: *mut libc::c_int, readable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn dtls_setup(vpninfo: *mut openconnect_info,
                  dtls_attempt_period: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cstp_obtain_cookie(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn cstp_common_headers(vpninfo: *mut openconnect_info,
                           buf: *mut oc_text_buf);
    #[no_mangle]
    fn cstp_mainloop(vpninfo: *mut openconnect_info,
                     timeout: *mut libc::c_int, readable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn cstp_connect(vpninfo: *mut openconnect_info) -> libc::c_int;
    #[no_mangle]
    fn cstp_bye(vpninfo: *mut openconnect_info, reason: *const libc::c_char)
     -> libc::c_int;
    /* Look up MSGID in the DOMAINNAME message catalog for the current CATEGORY
   locale.  */
    /* Similar to 'gettext' but select the plural form corresponding to the
   number N.  */
    /* Similar to 'dgettext' but select the plural form corresponding to the
   number N.  */
    /* Similar to 'dcgettext' but select the plural form corresponding to the
   number N.  */
    /* Set the current default message catalog to DOMAINNAME.
   If DOMAINNAME is null, return the current default.
   If DOMAINNAME is "", reset to the default of "messages".  */
    /* Specify that the DOMAINNAME message catalog will be found
   in DIRNAME rather than in the system locale data base.  */
    #[no_mangle]
    fn libintl_bindtextdomain(__domainname: *const libc::c_char,
                              __dirname: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn openconnect_create_useragent(base: *const libc::c_char)
     -> *mut libc::c_char;
    /* codeset name */
    /* string for formatting date and time */
    /* date format string */
    /* time format string */
    /* a.m. or p.m. time formatting string */
    /* Ante Meridian affix */
    /* Post Meridian affix */
    /* week day names */
    /* abbreviated week day names */
    /* month names */
    /* abbreviated month names */
    /* era description segments */
    /* era date format string */
    /* era date and time format string */
    /* era time format string */
    /* alternative symbols for digits */
    /* radix char */
    /* separator for thousands */
    /* affirmative response expression */
    /* negative response expression */
    /* affirmative response for yes/no queries */
    /* negative response for yes/no queries */
    /* currency symbol */
    /* month/day order (local extension) */
    #[no_mangle]
    fn nl_langinfo(_: nl_item) -> *mut libc::c_char;
    #[no_mangle]
    fn release_pcsc_ctx(info: *mut openconnect_info);
    #[no_mangle]
    fn free_pass(p: *mut *mut libc::c_char);
    /* The PKCS#8 data types */
    /* encrypted pub key */
    /* When decrypted, the following will not be NULL */
    /* used to encrypt and decrypt */
    /* true if we should auto free key_data */
    /* expanded version of 'enc_algor' */
    /*
 * The next 2 structures and their 8 routines were sent to me by Pat Richard
 * <patr@x509.com> and are used to manipulate Netscapes spki structures -
 * useful if you are writing a CA web page
 */
    /* challenge sent in atlas >= PR2 */
    /* signed public key and challenge */
    /* Netscape certificate sequence structure */
    /*- Unused (and iv length is wrong)
typedef struct CBCParameter_st
        {
        unsigned char iv[8];
        } CBC_PARAM;
*/
    /* Password based encryption structure */
    /* Password based encryption V2 structures */
    /* Usually OCTET STRING but could be anything */
    /* PKCS#8 private key info structure */
    /* Flag for various broken formats */
    /* Should be OCTET STRING but some are broken */
    /* #define      X509_get_serialNumber(x) ((x)->cert_info->serialNumber) */
    /* ****/
    /*
 * This one is only used so that a binary form can output, as in
 * i2d_X509_NAME(X509_get_X509_PUBKEY(x),&buf)
 */
    #[no_mangle]
    fn X509_free(a: *mut X509);
    #[no_mangle]
    fn free_split_routes(vpninfo: *mut openconnect_info);
    /* Retrieve an engine from the list by its unique "id" value. */
    #[no_mangle]
    fn ENGINE_by_id(id: *const libc::c_char) -> *mut ENGINE;
    #[no_mangle]
    fn ENGINE_load_builtin_engines();
    #[no_mangle]
    fn ENGINE_free(e: *mut ENGINE) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
/* !__GNUC__ */
/* !__GNUC__ */
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
/* [???] signal set */
pub type __darwin_suseconds_t = __int32_t;
/* [???] microseconds */
pub type __darwin_uid_t = __uint32_t;
/*
 * Copyright (c) 2004, 2008, 2009 Apple Inc. All rights reserved.
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
/* __uint32_t */
pub type __darwin_nl_item = libc::c_int;
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
pub type intptr_t = __darwin_intptr_t;
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
pub type uint32_t = libc::c_uint;
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
pub type time_t = __darwin_time_t;
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
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
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
pub type alloc_func
    =
    Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
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
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
/*
 * Copyright (c) 2000-2019 Apple Inc. All rights reserved.
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
/* Copyright (c) 1998, 1999 Apple Computer, Inc. All Rights Reserved */
/* Copyright (c) 1995 NeXT Computer, Inc. All Rights Reserved */
/*
 * Copyright (c) 1982, 1985, 1986, 1988, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 *	@(#)socket.h	8.4 (Berkeley) 2/21/94
 * $FreeBSD: src/sys/sys/socket.h,v 1.39.2.7 2001/07/03 11:02:01 ume Exp $
 */
/*
 * NOTICE: This file was modified by SPARTA, Inc. in 2005 to introduce
 * support for mandatory and extensible security protections.  This notice
 * is included in support of clause 2.2 (b) of the Apple Public License,
 * Version 2.0.
 */
/*
 * Definitions related to sockets: types, address families, options.
 */
/*
 * Data types.
 */
/* XXX Not explicitly defined by POSIX, but function return types are */
/* XXX Not explicitly defined by POSIX, but function return types are */
/*
 * [XSI] The iovec structure shall be defined as described in <sys/uio.h>.
 */
/*
 * Types
 */
/* stream socket */
/* datagram socket */
/* raw-protocol interface */
/* reliably-delivered message */
/* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
/* sequenced packet stream */
/*
 * Option flags per-socket.
 */
/* turn on debugging info recording */
/* socket has had listen() */
/* allow local address reuse */
/* keep connections alive */
/* just use interface addresses */
/* permit sending of broadcast msgs */
/* bypass hardware when possible */
/* linger on close if data present (in ticks) */
/* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
/* leave received OOB data in line */
/* allow local address & port reuse */
/* timestamp received dgram traffic */
/* Monotonically increasing timestamp on rcvd dgram */
/* APPLE: Retain unread data */
                                        /*  (ATOMIC proto) */
/* APPLE: Give hint when more data ready */
/* APPLE: Want OOB in MSG_FLAG on receive */
/* (!__APPLE__) */
/* (!_POSIX_C_SOURCE || _DARWIN_C_SOURCE) */
/*
 * Additional options, not kept in so_options.
 */
/* send buffer size */
/* receive buffer size */
/* send low-water mark */
/* receive low-water mark */
/* send timeout */
/* receive timeout */
/* get error status and clear */
/* get socket type */
/* socket's MAC label */
/* socket's peer MAC label */
/* APPLE: get 1st-packet byte count */
/* APPLE: Install socket-level NKE */
/* APPLE: No SIGPIPE on EPIPE */
/* APPLE: Returns EADDRNOTAVAIL when src is not available anymore */
/* APPLE: Get number of bytes currently in send socket buffer */
/* APPLE: Allow reuse of port/socket by different userids */
/* APPLE: send notification if there is a bind on a port which is already in use */
/* APPLE: block on close until an upcall returns */
/* linger on close if data present (in seconds) */
/* APPLE: request local port randomization */
/* To turn off some POSIX behavior */
/* number of datagrams in receive socket buffer */
/* Network service type */
/* Get QoS marking in effect for socket */
/*
 * Network Service Type for option SO_NET_SERVICE_TYPE
 *
 * The vast majority of sockets should use Best Effort that is the default
 * Network Service Type. Other Network Service Types have to be used only if
 * the traffic actually matches the description of the Network Service Type.
 *
 * Network Service Types do not represent priorities but rather describe
 * different categories of delay, jitter and loss parameters.
 * Those parameters may influence protocols from layer 4 protocols like TCP
 * to layer 2 protocols like Wi-Fi. The Network Service Type can determine
 * how the traffic is queued and scheduled by the host networking stack and
 * by other entities on the network like switches and routers. For example
 * for Wi-Fi, the Network Service Type can select the marking of the
 * layer 2 packet with the appropriate WMM Access Category.
 *
 * There is no point in attempting to game the system and use
 * a Network Service Type that does not correspond to the actual
 * traffic characteristic but one that seems to have a higher precedence.
 * The reason is that for service classes that have lower tolerance
 * for delay and jitter, the queues size is lower than for service
 * classes that are more tolerant to delay and jitter.
 *
 * For example using a voice service type for bulk data transfer will lead
 * to disastrous results as soon as congestion happens because the voice
 * queue overflows and packets get dropped. This is not only bad for the bulk
 * data transfer but it is also bad for VoIP apps that legitimately are using
 * the voice  service type.
 *
 * The characteristics of the Network Service Types are based on the service
 * classes defined in RFC 4594 "Configuration Guidelines for DiffServ Service
 * Classes"
 *
 * When system detects the outgoing interface belongs to a DiffServ domain
 * that follows the recommendation of the IETF draft "Guidelines for DiffServ to
 * IEEE 802.11 Mapping", the packet will marked at layer 3 with a DSCP value
 * that corresponds to Network Service Type.
 *
 * NET_SERVICE_TYPE_BE
 *	"Best Effort", unclassified/standard.  This is the default service
 *	class and cover the majority of the traffic.
 *
 * NET_SERVICE_TYPE_BK
 *	"Background", high delay tolerant, loss tolerant. elastic flow,
 *	variable size & long-lived. E.g: non-interactive network bulk transfer
 *	like synching or backup.
 *
 * NET_SERVICE_TYPE_RD
 *	"Responsive Data", a notch higher than "Best Effort", medium delay
 *	tolerant, elastic & inelastic flow, bursty, long-lived. E.g. email,
 *	instant messaging, for which there is a sense of interactivity and
 *	urgency (user waiting for output).
 *
 * NET_SERVICE_TYPE_OAM
 *	"Operations, Administration, and Management", medium delay tolerant,
 *	low-medium loss tolerant, elastic & inelastic flows, variable size.
 *	E.g. VPN tunnels.
 *
 * NET_SERVICE_TYPE_AV
 *	"Multimedia Audio/Video Streaming", medium delay tolerant, low-medium
 *	loss tolerant, elastic flow, constant packet interval, variable rate
 *	and size. E.g. video and audio playback with buffering.
 *
 * NET_SERVICE_TYPE_RV
 *	"Responsive Multimedia Audio/Video", low delay tolerant, low-medium
 *	loss tolerant, elastic flow, variable packet interval, rate and size.
 *	E.g. screen sharing.
 *
 * NET_SERVICE_TYPE_VI
 *	"Interactive Video", low delay tolerant, low-medium loss tolerant,
 *	elastic flow, constant packet interval, variable rate & size. E.g.
 *	video telephony.
 *
 * NET_SERVICE_TYPE_SIG
 *	"Signaling", low delay tolerant, low loss tolerant, inelastic flow,
 *	jitter tolerant, rate is bursty but short, variable size. E.g. SIP.
 *
 * NET_SERVICE_TYPE_VO
 *	"Interactive Voice", very low delay tolerant, very low loss tolerant,
 *	inelastic flow, constant packet rate, somewhat fixed size.
 *	E.g. VoIP.
 */
/* Best effort */
/* Background system initiated */
/* Signaling */
/* Interactive Video */
/* Interactive Voice */
/* Responsive Multimedia Audio/Video */
/* Multimedia Audio/Video Streaming */
/* Operations, Administration, and Management */
/* Responsive Data */
/* These are supported values for SO_NETSVC_MARKING_LEVEL */
/* The outgoing network interface is not known */
/* Default marking at layer 2 (for example Wi-Fi WMM) */
/* Layer 3 DSCP marking and layer 2 marking for all Network Service Types */
/* The system policy limits layer 3 DSCP marking and layer 2 marking
	                                         * to background Network Service Types */
/* connectx() flag parameters */
/* resume connect() on read/write */
/* data is idempotent */
/* data includes security that replaces the TFO-cookie */
/* sockaddr endpoints */
/* optional source interface */
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
}
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct oc_vpn_proto {
    pub name: *const libc::c_char,
    pub pretty_name: *const libc::c_char,
    pub description: *const libc::c_char,
    pub flags: libc::c_uint,
}
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
}
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
}
/* ssl/ssl.h */
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
 * Copyright (c) 1998-2018 The OpenSSL Project.  All rights reserved.
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
/* ====================================================================
 * Copyright 2005 Nokia. All rights reserved.
 *
 * The portions of the attached software ("Contribution") is developed by
 * Nokia Corporation and is licensed pursuant to the OpenSSL open source
 * license.
 *
 * The Contribution, originally written by Mika Kousa and Pasi Eronen of
 * Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
 * support (see RFC 4279) to OpenSSL.
 *
 * No patent licenses or other rights except those expressly stated in
 * the OpenSSL open source license shall be deemed granted or received
 * expressly, by implication, estoppel, or otherwise.
 *
 * No assurances are provided by Nokia that the Contribution does not
 * infringe the patent or other intellectual property rights of any third
 * party or that the license provides you with all the necessary rights
 * to make use of the Contribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND. IN
 * ADDITION TO THE DISCLAIMERS INCLUDED IN THE LICENSE, NOKIA
 * SPECIFICALLY DISCLAIMS ANY LIABILITY FOR CLAIMS BROUGHT BY YOU OR ANY
 * OTHER ENTITY BASED ON INFRINGEMENT OF INTELLECTUAL PROPERTY RIGHTS OR
 * OTHERWISE.
 */
/* SSLeay version number for ASN.1 encoding of the session information */
/*-
 * Version 0 - initial version
 * Version 1 - added the optional peer certificate
 */
/* text strings for the ciphers */
/*
 * VRS Additional Kerberos5 entries
 */
/* These are used to specify which ciphers to use and not to use */
/* unused! */
/* unused! */
/* unused! */
/* unused! */
/* alias for kEDH */
/* alias for kEECDH */
/* same as "kEDH:-ADH" */
/* alias for EDH */
/* same as "kEECDH:-AECDH" */
/* alias for ECDHE" */
/* same as "SHA1" */
/*-
 * COMPLEMENTOF* definitions. These identifiers are used to (de-select)
 * ciphers normally not being used.
 * Example: "RC4" will activate all ciphers using RC4 including ciphers
 * without authentication, which would normally disabled by DEFAULT (due
 * the "!ADH" being part of default). Therefore "RC4:!COMPLEMENTOFDEFAULT"
 * will make sure that it is also disabled in the specific selection.
 * COMPLEMENTOF* identifiers are portable between version, as adjustments
 * to the default cipher setup will also be included here.
 *
 * COMPLEMENTOFDEFAULT does not experience the same special treatment that
 * DEFAULT gets, as only selection is being done and no sorting as needed
 * for DEFAULT.
 */
/*
 * The following cipher list is used by default. It also is substituted when
 * an application-defined cipher list string starts with 'DEFAULT'.
 */
/*
 * As of OpenSSL 1.0.0, ssl_create_cipher_list() in ssl/ssl_ciph.c always
 * starts with a reasonable order, and all we have to do for DEFAULT is
 * throwing out anonymous and unencrypted ciphersuites! (The latter are not
 * actually enabled by ALL, but "ALL:RSA" would enable some of them.)
 */
/* Used in SSL_set_shutdown()/SSL_get_shutdown(); */
/*
 * This is needed to stop compilers complaining about the 'struct ssl_st *'
 * function parameters used to prototype callbacks in SSL_CTX.
 */
/* SRTP protection profiles for use with the use_srtp extension (RFC 5764)*/
/* Typedefs for handling custom extensions */
/* used to hold info on the particular ciphers used */
/* text name */
/* id, 4 bytes, first is version */
/*
     * changed in 0.9.9: these four used to be portions of a single value
     * 'algorithms'
     */
/* key exchange algorithm */
/* server authentication */
/* symmetric encryption */
/* symmetric authentication */
/* (major) protocol version */
/* strength and export flags */
/* Extra flags */
/* Number of bits really used */
/* Number of bits for algorithm */
/* Used to hold functions for SSLv2 or SSLv3/TLSv1 functions */
/* Extra SSLv3/TLS stuff */
/*-
 * Lets make this into an ASN.1 type structure as follows
 * SSL_SESSION_ID ::= SEQUENCE {
 *      version                 INTEGER,        -- structure version number
 *      SSLversion              INTEGER,        -- SSL version number
 *      Cipher                  OCTET STRING,   -- the 3 byte cipher ID
 *      Session_ID              OCTET STRING,   -- the Session ID
 *      Master_key              OCTET STRING,   -- the master key
 *      KRB5_principal          OCTET STRING    -- optional Kerberos principal
 *      Key_Arg [ 0 ] IMPLICIT  OCTET STRING,   -- the optional Key argument
 *      Time [ 1 ] EXPLICIT     INTEGER,        -- optional Start Time
 *      Timeout [ 2 ] EXPLICIT  INTEGER,        -- optional Timeout ins seconds
 *      Peer [ 3 ] EXPLICIT     X509,           -- optional Peer Certificate
 *      Session_ID_context [ 4 ] EXPLICIT OCTET STRING,   -- the Session ID context
 *      Verify_result [ 5 ] EXPLICIT INTEGER,   -- X509_V_... code for `Peer'
 *      HostName [ 6 ] EXPLICIT OCTET STRING,   -- optional HostName from servername TLS extension
 *      PSK_identity_hint [ 7 ] EXPLICIT OCTET STRING, -- optional PSK identity hint
 *      PSK_identity [ 8 ] EXPLICIT OCTET STRING,  -- optional PSK identity
 *      Ticket_lifetime_hint [9] EXPLICIT INTEGER, -- server's lifetime hint for session ticket
 *      Ticket [10]             EXPLICIT OCTET STRING, -- session ticket (clients only)
 *      Compression_meth [11]   EXPLICIT OCTET STRING, -- optional compression method
 *      SRP_username [ 12 ] EXPLICIT OCTET STRING -- optional SRP username
 *      }
 * Look in ssl/ssl_asn1.c for more details
 * I'm using EXPLICIT tags so I can read the damn things using asn1parse :-).
 */
/* what ssl version session info is being
                                 * kept in here? */
/* only really used in SSLv2 */
/* session_id - valid? */
/*
     * this is used to determine whether the session is being reused in the
     * appropriate context. It is up to the application to set this, via
     * SSL_new
     */
/* OPENSSL_NO_KRB5 */
/*
     * Used to indicate that session resumption is not allowed. Applications
     * can also set this bit for a new session via not_resumable_session_cb
     * to disable session caching and tickets.
     */
/* The cert is the certificate used to establish this connection */
/* SESS_CERT */
/*
     * This is the cert for the other end. On clients, it will be the same as
     * sess_cert->peer_key->x509 (the latter is not enough as sess_cert is
     * not retained in the external representation of sessions, see
     * ssl_asn1.c).
     */
/*
     * when app_verify_callback accepts a session where the peer's
     * certificate is not ok, we must remember the error for session reuse:
     */
/* only for servers */
/* Need to lookup the method */
/* when ASN.1 loaded, this needs to be used
                                 * to load the 'cipher' structure */
/* ciphers offered by the client */
/* application specific data */
/*
     * These are used to make removal of session-ids more efficient and to
     * implement a maximum cache size.
     */
/* peer's list */
/* peer's list */
/* OPENSSL_NO_EC */
/* RFC4507 info */
/* Session ticket */
/* Session ticket length */
/* Session lifetime hint in seconds */
/* Allow initial connection to servers that don't support RI */
/* Hasn't done anything since OpenSSL 0.9.7h, retained for compatibility */
/* Refers to ancient SSLREF and SSLv2, retained for compatibility */
/*
 * Disable SSL 3.0/TLS 1.0 CBC vulnerability workaround that was added in
 * OpenSSL 0.9.6d.  Usually (depending on the application protocol) the
 * workaround is not needed.  Unfortunately some broken SSL/TLS
 * implementations cannot handle it at all, which is why we include it in
 * SSL_OP_ALL.
 */
/* added in 0.9.6e */
/*
 * SSL_OP_ALL: various bug workarounds that should be rather harmless.  This
 * used to be 0x000FFFFFL before 0.9.7.
 */
/* DTLS options */
/* Turn on Cookie Exchange (on relevant for servers) */
/* Don't use RFC4507 ticket extension */
/* Use Cisco's "speshul" version of DTLS_BAD_VER (as client)  */
/* As server, disallow session resumption on renegotiation */
/* Don't use compression even if supported */
/* Permit unsafe legacy renegotiation */
/* If set, always create a new key when using tmp_ecdh parameters */
/* Does nothing: retained for compatibility */
/* Does nothing: retained for compatibiity */
/*
 * Set on servers to choose the cipher according to the server's preferences
 */
/*
 * If set, a server will allow a client to issue a SSLv3.0 version number as
 * latest version supported in the premaster secret, even when TLSv1.0
 * (version 3.1) was announced in the client hello. Normally this is
 * forbidden to prevent version rollback attacks.
 */
/*
 * These next two were never actually used for anything since SSLeay zap so
 * we have some more flags.
 */
/*
 * The next flag deliberately changes the ciphertest, this is a check for the
 * PKCS#1 attack
 */
/*
 * Make server add server-hello extension from early version of cryptopro
 * draft, when GOST ciphersuite is negotiated. Required for interoperability
 * with CryptoPro CSP 3.x
 */
/*
 * Allow SSL_write(..., n) to return r with 0 < r < n (i.e. report success
 * when just a single record has been written):
 */
/*
 * Make it possible to retry SSL_write() with changed buffer location (buffer
 * contents must stay the same!); this is not the default to avoid the
 * misconception that non-blocking SSL_write() behaves like non-blocking
 * write():
 */
/*
 * Never bother the application with retries if the transport is blocking:
 */
/* Don't attempt to automatically build certificate chain */
/*
 * Save RAM by releasing read and write buffers when they're empty. (SSL3 and
 * TLS only.) "Released" buffers are put onto a free-list in the context or
 * just freed (depending on the context's setting for freelist_max_len).
 */
/*
 * Send the current time in the Random fields of the ClientHello and
 * ServerHello records for compatibility with hypothetical implementations
 * that require it.
 */
/*
 * Send TLS_FALLBACK_SCSV in the ClientHello. To be set only by applications
 * that reconnect with a downgraded protocol version; see
 * draft-ietf-tls-downgrade-scsv-00 for details. DO NOT ENABLE THIS if your
 * application attempts a normal handshake. Only use this in explicit
 * fallback retries, following the guidance in
 * draft-ietf-tls-downgrade-scsv-00.
 */
/* Cert related flags */
/*
 * Many implementations ignore some aspects of the TLS standards such as
 * enforcing certifcate chain algorithms. When this is set we enforce them.
 */
/* Suite B modes, takes same values as certificate verify flags */
/* Suite B 192 bit only mode */
/* Suite B 128 bit mode allowing 192 bit algorithms */
/* Perform all sorts of protocol violations for testing purposes */
/* Flags for building certificate chains */
/* Treat any existing certificates as untrusted CAs */
/* Don't include root CA in chain */
/* Just check certificates already there */
/* Ignore verification errors */
/* Clear verification errors from queue */
/* Flags returned by SSL_check_chain */
/* Certificate can be used with this session */
/* Certificate can also be used for signing */
/* EE certificate signing algorithm OK */
/* CA signature algorithms OK */
/* EE certificate parameters OK */
/* CA certificate parameters OK */
/* Signing explicitly allowed as opposed to SHA1 fallback */
/* Client CA issuer names match (always set for server cert) */
/* Cert type matches client types (always set for server cert) */
/* Cert chain suitable to Suite B */
/* Configuration value types */
/*
 * Note: SSL[_CTX]_set_{options,mode} use |= op on the previous value, they
 * cannot be used to clear bits.
 */
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
/* param for all the callbacks */
/* set client Hello login callback */
/* set SRP N/g param callback for verification */
/* set SRP client passwd callback */
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
pub type BIGNUM = bignum_st;
/* crypto/bn/bn.h */
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
/* ====================================================================
 * Copyright (c) 1998-2018 The OpenSSL Project.  All rights reserved.
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
 * The Contribution is licensed pursuant to the Eric Young open source
 * license provided above.
 *
 * The binary polynomial arithmetic software is originally written by
 * Sheueling Chang Shantz and Douglas Stebila of Sun Microsystems Laboratories.
 *
 */
/* FILE */
/*
 * These preprocessor symbols control various aspects of the bignum headers
 * and library code. They're not defined by any "normal" configuration, as
 * they are intended for development and testing purposes. NB: defining all
 * three can be useful for debugging application code as well as openssl
 * itself. BN_DEBUG - turn on various debugging alterations to the bignum
 * code BN_DEBUG_RAND - uses random poisoning of unused words to trip up
 * mismanagement of bignum internals. You must also define BN_DEBUG.
 */
/* #define BN_DEBUG */
/* #define BN_DEBUG_RAND */
/*
 * This next option uses the C libraries (2 word)/(1 word) function. If it is
 * not defined, I use my C version (which is slower). The reason for this
 * flag is that when the particular C compiler library routine is used, and
 * the library is linked with a different compiler, the library is missing.
 * This mostly happens when the library is built with gcc and then linked
 * using normal cc.  This would be a common occurrence because gcc normally
 * produces code that is 2 times faster than system compilers for the big
 * number stuff. For machines with only one compiler (or shared libraries),
 * this should be on.  Again this in only really a problem on machines using
 * "long long's", are 32bit, and are not using my assembler code.
 */
/*
 * assuming long is 64bit - this is the DEC Alpha unsigned long long is only
 * 64 bits :-(, don't define BN_LLONG for the DEC Alpha
 */
/*
 * This is where the long long data type is 64 bits, but long is 32. For
 * machines where there are 64bit registers, this is the mode to use. IRIX,
 * on R4000 and above should use this mode, along with the relevant assembler
 * code :-).  Do NOT define BN_LLONG.
 */
/*
 * avoid leaking exponent information through timing,
 * BN_mod_exp_mont() will call BN_mod_exp_mont_consttime,
 * BN_div() will call BN_div_no_branch,
 * BN_mod_inverse() will call BN_mod_inverse_no_branch.
 */
/* used for debuging */
/*
 * get a clone of a BIGNUM with changed flags, for *temporary* use only (the
 * two BIGNUMs cannot not be used in parallel!)
 */
/* Already declared in ossl_typ.h */
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
    /* If placed in pkcs12.h, we end up with a circular depency with pkcs7.h */
    /* Nothing */
    /* Nothing */
    /* Callback types for crypto.h */
    /* def HEADER_OPENSSL_TYPES_H */
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
/* the divisor */
/* the reciprocal */
/* Used for slow "generation" functions. */
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
/* Pointer to an array of 'BN_BITS2' bit
                                 * chunks. */
/* Index of last used d +1. */
/* The next are internal book keeping for bn_expand. */
/* Size of the d array. */
/* one if the number is negative */
/* Used for montgomery multiplication */
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
/* X509_CRL */
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
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2015 Intel Corporation.
 * Copyright © 2008 Nick Andrew <nick@nick-andrew.net>
 * Copyright © 2013 John Morrissey <jwm@horde.net>
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
/* Ick */
/* OPENSSL */
/* FreeBSD provides this in <sys/param.h>  */
/* At least MinGW headers seem not to provide IPPROTO_IPIP */
/* ***************************************************************************/
/* Random secret has not been generated yet */
/* Secret is present, ready to attempt DTLS */
/* DTLS was disabled on the *client* side */
/* For ESP, sometimes sending probes */
/* ESP probe received; must tell server */
/* Server informed and should be sending ESP */
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
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2016 Intel Corporation.
 * Copyright © 2008 Nick Andrew <nick@nick-andrew.net>
 * Copyright © 2013 John Morrissey <jwm@horde.net>
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
 * API version 5.5 (v8.00; 2019-01-05):
 *  - add openconnect_set_version_string()
 *  - add openconnect_set_key_password()
 *  - Add openconnect_has_tss2_blob_support()
 *  - Add openconnect_get_supported_protocols()
 *  - Add openconnect_free_supported_protocols()
 *  - Add openconnect_get_protocol()
 *  - Add openconnect_get_idle_timeout()
 *
 * API version 5.4 (v7.08; 2016-12-13):
 *  - Add openconnect_set_pass_tos()
 *
 * API version 5.3 (v7.07; 2016-07-11):
 *  - Add openconnect_set_localname().
 *  - Add openconnect_override_getaddrinfo().
 *  - Add openconnect_get_cstp_compression().
 *  - Add openconnect_get_dtls_compression().
 *  - Add openconnect_disable_ipv6().
 *  - Add ip_info->gateway_addr.
 *  - Add openconnect_set_setup_tun_handler().
 *  - Add openconnect_set_reconnected_handler().
 *  - Add openconnect_get_dnsname().
 *  - Add openconnect_get_peer_cert_chain() and
 *        openconnect_free_peer_cert_chain().
 *
 * API version 5.2 (v7.05; 2015-03-10):
 *  - Add openconnect_set_http_auth(), openconnect_set_protocol().
 *
 * API version 5.1 (v7.05; 2015-03-10):
 *  - Add openconnect_set_compression_mode(), openconnect_set_loglevel()
 *
 *   (Note: API 5.1 and openconnect_set_compression_mode() were present in
 *    this file in the v7.04 release on 2015-01-25, but the symbol versioning
 *    for the new function was OPENCONNECT_5_0, and openconnect_set_loglevel()
 *    was not yet present.)
 *
 * API version 5.0 (v7.00; 2014-11-27):
 *  - Remove OPENCONNECT_X509 and openconnect_get_peer_cert().
 *  - Change openconnect_get_cert_der() to openconnect_get_peer_cert_DER() etc.
 *  - Add openconnect_check_peer_cert_hash().
 *  - Remove openconnect_set_server_cert_sha1().
 *  - Add openconnect_has_yubioath_support() and OC_TOKEN_MODE_YUBIOATH.
 *  - Add openconnect_has_system_key_support().
 *
 * API version 4.1 (v7.00; 2014-11-27):
 *  - Add openconnect_get_cstp_cipher(), openconnect_get_dtls_cipher(),
 *    openconnect_set_system_trust(), openconnect_set_csd_environ().
 *  - Change openconnect_init_ssl() to return int.
 *
 * API version 4.0 (v7.00; 2014-11-27):
 *  - Change string handling to never transfer ownership of allocations.
 *  - Add openconnect_set_option_value(), openconnect_free_cert_info().
 *
 * API version 3.4 (v7.00; 2014-11-27):
 *  - Add openconnect_set_token_callbacks()
 *
 * API version 3.3 (v6.00; 2014-07-08):
 *  - Add openconnect_set_pfs(), openconnect_set_dpd(),
 *    openconnect_set_proxy_auth()
 *
 * API version 3.2 (v5.99; 2014-03-05):
 *  - Add OC_TOKEN_MODE_HOTP and allow openconnect_has_oath_support() to
 *    return 2 to indicate that it is present.
 *
 * API version 3.1 (v5.99; 2014-03-05):
 *  - Add openconnect_setup_cmd_pipe(), openconnect_mainloop(),
 *    openconnect_setup_tun_device(), openconnect_setup_tun_script(),
 *    openconnect_setup_tun_fd(), openconnect_setup_dtls(),
 *    openconnect_make_cstp_connection(), openconnect_set_server_cert_sha1(),
 *    openconnect_get_ifname(), openconnect_set_reqmtu(),
 *    openconnect_get_ip_info(), openconnect_set_protect_socket_handler(),
 *    openconnect_set_mobile_info(), openconnect_set_xmlpost(),
 *    openconnect_set_stats_handler()
 *
 * API version 3.0 (v5.99; 2014-03-05):
 *  - Change oc_form_opt_select->choices to an array of pointers
 *  - Add oc_form_opt->flags
 *  - Add OC_FORM_RESULT_* and oc_auth_form->authgroup_*
 *
 * API version 2.2 (v5.00; 2013-05-16):
 *  - Add openconnect_set_token_mode(), openconnect_has_oath_support()
 *  - Deprecate openconnect_set_stoken_mode()
 *
 * API version 2.1 (v4.99; 2013-02-07):
 *  - Add openconnect_set_reported_os()
 *  - Add openconnect_set_stoken_mode(), openconnect_has_stoken_support()
 *
 * API version 2.0 (v3.99; 2012-06-13):
 *  - OPENCONNECT_X509 is now an opaque type.
 *  - Add openconnect_has_pkcs11_support(), openconnect_has_tss_blob_support()
 *  - Rename openconnect_init_openssl() -> openconnect_init_ssl()
 *  - Rename openconnect_vpninfo_new_with_cbdata() -> openconnect_vpninfo_new()
 *    and kill the old openconnect_vpninfo_new() and its callback types.
 *
 * API version 1.5 (v3.99; 2012-06-13):
 *  - Add openconnect_get_cert_details(), openconnect_get_cert_DER().
 *
 * API version 1.4 (v3.19; 2012-05-17):
 *  - Add openconnect_set_cancel_fd()
 *
 * API version 1.3 (v3.13; 2011-09-30):
 *  - Add openconnect_set_cert_expiry_warning() to change from default 60 days
 *
 * API version 1.2 (v3.10; 2011-06-30)
 *  - Add openconnect_vpninfo_new_with_cbdata()
 *
 * API version 1.1 (v3.02; 2011-04-19):
 *  - Add openconnect_vpninfo_free()
 *
 * API version 1.0 (v3.00; 2011-03-09):
 *  - Initial version
 *
 * NEW LIBRARY FUNCTION CHECKLIST:
 *
 * 1) Bump the API version if the current API version has already appeared
 *    in a release
 * 2) Add function to the above changelog
 * 3) Add function to libopenconnect.map.in
 * 4) Add declaration + comments in the latter part of this file
 * 5) Add function to jni.c, then test with ./configure --with-java && make
 * 6) Add declaration to LibOpenConnect.java, then run "cd java && ant" to test
 */
/* Before API version 1.4 (OpenConnect 3.19) this macro didn't exist.
 * Somewhat ironic, that the API version check itself needs to be
 * conditionally used depending on the API version. A very simple way
 * for users to handle this with an approximately correct answer is
 *   #include <openconnect.h>
 *   #ifndef OPENCONNECT_CHECK_VER
 *   #define OPENCONNECT_CHECK_VER(x,y) 0
 *   #endif
 */
/* ***************************************************************************/
/* Enumeration of supported VPN protocols */
/* ***************************************************************************/
/* Authentication form processing */
/* char * fields are static (owned by XML parser) and don't need to be
   freed by the form handling code — except for value, which for TEXT
   and PASSWORD options is allocated by openconnect_set_option_value()
   when process_form() interacts with the user and must be freed. */
/* Use openconnect_set_option_value() to set this */
/* To set the value to a form use the following function */
/* All fields are static, owned by the XML parser */
/* All char * fields are static, owned by the XML parser */
/* The elements above this line come from server-provided CSTP headers,
	 * so they should be handled with caution.  gateway_addr is generated
	 * locally from getnameinfo(). */
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
pub type oc_token_mode_t = libc::c_uint;
pub const OC_TOKEN_MODE_YUBIOATH: oc_token_mode_t = 4;
pub const OC_TOKEN_MODE_HOTP: oc_token_mode_t = 3;
pub const OC_TOKEN_MODE_TOTP: oc_token_mode_t = 2;
pub const OC_TOKEN_MODE_STOKEN: oc_token_mode_t = 1;
pub const OC_TOKEN_MODE_NONE: oc_token_mode_t = 0;
pub type oc_compression_mode_t = libc::c_uint;
pub const OC_COMPRESSION_MODE_ALL: oc_compression_mode_t = 2;
pub const OC_COMPRESSION_MODE_STATELESS: oc_compression_mode_t = 1;
pub const OC_COMPRESSION_MODE_NONE: oc_compression_mode_t = 0;
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
pub type nl_item = __darwin_nl_item;
#[inline]
unsafe extern "C" fn set_sock_nonblock(mut fd: libc::c_int) -> libc::c_int {
    return fcntl(fd, 4i32, fcntl(fd, 3i32) | 0x4i32);
}
#[inline]
unsafe extern "C" fn init_pkt_queue(mut q: *mut pkt_q) {
    (*q).tail = &mut (*q).head;
}
/*
 * OpenConnect (SSL + DTLS) VPN client
 *
 * Copyright © 2008-2015 Intel Corporation.
 * Copyright © 2013 John Morrissey <jwm@horde.net>
 *
 * Authors: David Woodhouse <dwmw2@infradead.org>
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
#[no_mangle]
pub unsafe extern "C" fn openconnect_vpninfo_new(mut useragent:
                                                     *const libc::c_char,
                                                 mut validate_peer_cert:
                                                     openconnect_validate_peer_cert_vfn,
                                                 mut write_new_config:
                                                     openconnect_write_new_config_vfn,
                                                 mut process_auth_form_0:
                                                     openconnect_process_auth_form_vfn,
                                                 mut progress:
                                                     openconnect_progress_vfn,
                                                 mut privdata:
                                                     *mut libc::c_void)
 -> *mut openconnect_info {
    let mut vpninfo: *mut openconnect_info =
        calloc(::std::mem::size_of::<openconnect_info>() as libc::c_ulong,
               1i32 as libc::c_ulong) as *mut openconnect_info;
    let mut charset: *mut libc::c_char = nl_langinfo(0i32);
    if vpninfo.is_null() { return 0 as *mut openconnect_info }
    if !charset.is_null() &&
           strcmp(charset, b"UTF-8\x00" as *const u8 as *const libc::c_char)
               != 0 {
        (*vpninfo).ic_utf8_to_legacy =
            iconv_open(charset,
                       b"UTF-8\x00" as *const u8 as *const libc::c_char);
        (*vpninfo).ic_legacy_to_utf8 =
            iconv_open(b"UTF-8\x00" as *const u8 as *const libc::c_char,
                       charset)
    } else {
        (*vpninfo).ic_utf8_to_legacy = -1i32 as iconv_t;
        (*vpninfo).ic_legacy_to_utf8 = -1i32 as iconv_t
    }
    (*vpninfo).tun_fd = -1i32;
    init_pkt_queue(&mut (*vpninfo).incoming_queue);
    init_pkt_queue(&mut (*vpninfo).outgoing_queue);
    init_pkt_queue(&mut (*vpninfo).oncp_control_queue);
    (*vpninfo).dtls_tos_current = 0i32;
    (*vpninfo).dtls_pass_tos = 0i32;
    (*vpninfo).dtls_fd = -1i32;
    (*vpninfo).ssl_fd = (*vpninfo).dtls_fd;
    (*vpninfo).cmd_fd_write = -1i32;
    (*vpninfo).cmd_fd = (*vpninfo).cmd_fd_write;
    (*vpninfo).tncc_fd = -1i32;
    (*vpninfo).cert_expire_warning = 60i32 * 86400i32;
    (*vpninfo).req_compr = 1i32 << 1i32 | 1i32 << 2i32 | 1i32 << 3i32;
    (*vpninfo).max_qlen = 10i32;
    (*vpninfo).localname =
        strdup(b"localhost\x00" as *const u8 as *const libc::c_char);
    (*vpninfo).useragent = openconnect_create_useragent(useragent);
    (*vpninfo).validate_peer_cert = validate_peer_cert;
    (*vpninfo).write_new_config = write_new_config;
    (*vpninfo).process_auth_form = process_auth_form_0;
    (*vpninfo).progress = progress;
    (*vpninfo).cbdata =
        if !privdata.is_null() {
            privdata
        } else { vpninfo as *mut libc::c_void };
    (*vpninfo).xmlpost = 1i32;
    (*vpninfo).verbose = 3i32;
    (*vpninfo).try_http_auth = 1i32;
    (*vpninfo).proxy_auth[3].state = -3i32;
    (*vpninfo).http_auth[3].state = -3i32;
    openconnect_set_reported_os(vpninfo, 0 as *const libc::c_char);
    if (*vpninfo).localname.is_null() || (*vpninfo).useragent.is_null() {
        free((*vpninfo).localname as *mut libc::c_void);
        free((*vpninfo).useragent as *mut libc::c_void);
        free(vpninfo as *mut libc::c_void);
        return 0 as *mut openconnect_info
    } else {
        libintl_bindtextdomain(b"openconnect\x00" as *const u8 as
                                   *const libc::c_char,
                               b"/usr/local/share/locale\x00" as *const u8 as
                                   *const libc::c_char);
        openconnect_set_protocol(vpninfo,
                                 b"anyconnect\x00" as *const u8 as
                                     *const libc::c_char);
        return vpninfo
    };
}
#[no_mangle]
pub static mut openconnect_protos: [vpn_proto; 5] =
    unsafe {
        [{
             let mut init =
                 vpn_proto{name:
                               b"anyconnect\x00" as *const u8 as
                                   *const libc::c_char,
                           pretty_name:
                               b"Cisco AnyConnect or openconnect\x00" as
                                   *const u8 as *const libc::c_char,
                           description:
                               b"Compatible with Cisco AnyConnect SSL VPN, as well as ocserv\x00"
                                   as *const u8 as *const libc::c_char,
                           udp_protocol:
                               b"DTLS\x00" as *const u8 as
                                   *const libc::c_char,
                           flags:
                               (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32 |
                                    1i32 << 3i32 | 1i32 << 4i32) as
                                   libc::c_uint,
                           vpn_close_session:
                               Some(cstp_bye as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *const libc::c_char)
                                            -> libc::c_int),
                           obtain_cookie:
                               Some(cstp_obtain_cookie as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_connect:
                               Some(cstp_connect as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_mainloop:
                               Some(cstp_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           add_http_headers:
                               Some(cstp_common_headers as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut oc_text_buf)
                                            -> ()),
                           udp_setup:
                               Some(dtls_setup as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_mainloop:
                               Some(dtls_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_close:
                               Some(dtls_close as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_shutdown:
                               Some(dtls_shutdown as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_send_probes: None,
                           udp_catch_probe: None,};
             init
         },
         {
             let mut init =
                 vpn_proto{name:
                               b"nc\x00" as *const u8 as *const libc::c_char,
                           pretty_name:
                               b"Juniper Network Connect\x00" as *const u8 as
                                   *const libc::c_char,
                           description:
                               b"Compatible with Juniper Network Connect\x00"
                                   as *const u8 as *const libc::c_char,
                           udp_protocol:
                               b"ESP\x00" as *const u8 as *const libc::c_char,
                           flags:
                               (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32 |
                                    1i32 << 3i32) as libc::c_uint,
                           vpn_close_session:
                               Some(oncp_bye as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *const libc::c_char)
                                            -> libc::c_int),
                           obtain_cookie:
                               Some(oncp_obtain_cookie as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_connect:
                               Some(oncp_connect as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_mainloop:
                               Some(oncp_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           add_http_headers:
                               Some(oncp_common_headers as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut oc_text_buf)
                                            -> ()),
                           udp_setup:
                               Some(esp_setup as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_mainloop:
                               Some(esp_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_close:
                               Some(oncp_esp_close as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_shutdown:
                               Some(esp_shutdown as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_send_probes:
                               Some(oncp_esp_send_probes as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           udp_catch_probe:
                               Some(oncp_esp_catch_probe as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _: *mut pkt)
                                            -> libc::c_int),};
             init
         },
         {
             let mut init =
                 vpn_proto{name:
                               b"gp\x00" as *const u8 as *const libc::c_char,
                           pretty_name:
                               b"Palo Alto Networks GlobalProtect\x00" as
                                   *const u8 as *const libc::c_char,
                           description:
                               b"Compatible with Palo Alto Networks (PAN) GlobalProtect SSL VPN\x00"
                                   as *const u8 as *const libc::c_char,
                           udp_protocol:
                               b"ESP\x00" as *const u8 as *const libc::c_char,
                           flags:
                               (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32 |
                                    1i32 << 3i32 | 1i32 << 4i32) as
                                   libc::c_uint,
                           vpn_close_session:
                               Some(gpst_bye as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *const libc::c_char)
                                            -> libc::c_int),
                           obtain_cookie:
                               Some(gpst_obtain_cookie as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_connect:
                               Some(gpst_setup as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_mainloop:
                               Some(gpst_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           add_http_headers:
                               Some(gpst_common_headers as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut oc_text_buf)
                                            -> ()),
                           udp_setup:
                               Some(esp_setup as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_mainloop:
                               Some(esp_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_close:
                               Some(esp_close as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_shutdown:
                               Some(esp_shutdown as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_send_probes:
                               Some(gpst_esp_send_probes as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           udp_catch_probe:
                               Some(gpst_esp_catch_probe as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _: *mut pkt)
                                            -> libc::c_int),};
             init
         },
         {
             let mut init =
                 vpn_proto{name:
                               b"pulse\x00" as *const u8 as
                                   *const libc::c_char,
                           pretty_name:
                               b"Pulse Connect Secure\x00" as *const u8 as
                                   *const libc::c_char,
                           description:
                               b"Compatible with Pulse Connect Secure SSL VPN\x00"
                                   as *const u8 as *const libc::c_char,
                           udp_protocol:
                               b"ESP\x00" as *const u8 as *const libc::c_char,
                           flags: (1i32 << 0i32) as libc::c_uint,
                           vpn_close_session:
                               Some(pulse_bye as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *const libc::c_char)
                                            -> libc::c_int),
                           obtain_cookie:
                               Some(pulse_obtain_cookie as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_connect:
                               Some(pulse_connect as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           tcp_mainloop:
                               Some(pulse_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           add_http_headers:
                               Some(http_common_headers as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut oc_text_buf)
                                            -> ()),
                           udp_setup:
                               Some(esp_setup as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_mainloop:
                               Some(esp_mainloop as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: libc::c_int)
                                            -> libc::c_int),
                           udp_close:
                               Some(esp_close as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_shutdown:
                               Some(esp_shutdown as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> ()),
                           udp_send_probes:
                               Some(oncp_esp_send_probes as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info)
                                            -> libc::c_int),
                           udp_catch_probe:
                               Some(oncp_esp_catch_probe as
                                        unsafe extern "C" fn(_:
                                                                 *mut openconnect_info,
                                                             _: *mut pkt)
                                            -> libc::c_int),};
             init
         },
         {
             let mut init =
                 vpn_proto{name: 0 as *const libc::c_char,
                           pretty_name: 0 as *const libc::c_char,
                           description: 0 as *const libc::c_char,
                           udp_protocol: 0 as *const libc::c_char,
                           flags: 0,
                           vpn_close_session: None,
                           obtain_cookie: None,
                           tcp_connect: None,
                           tcp_mainloop: None,
                           add_http_headers: None,
                           udp_setup: None,
                           udp_mainloop: None,
                           udp_close: None,
                           udp_shutdown: None,
                           udp_send_probes: None,
                           udp_catch_probe: None,};
             init
         }]
    };
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_supported_protocols(mut protos:
                                                                 *mut *mut oc_vpn_proto)
 -> libc::c_int {
    let mut pr: *mut oc_vpn_proto = 0 as *mut oc_vpn_proto;
    let mut p: *const vpn_proto = 0 as *const vpn_proto;
    pr =
        calloc((::std::mem::size_of::<[vpn_proto; 5]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<vpn_proto>()
                                                    as libc::c_ulong),
               ::std::mem::size_of::<oc_vpn_proto>() as libc::c_ulong) as
            *mut oc_vpn_proto;
    *protos = pr;
    if pr.is_null() { return -12i32 }
    p = openconnect_protos.as_ptr();
    while !(*p).name.is_null() {
        (*pr).name = (*p).name;
        (*pr).pretty_name =
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char, (*p).pretty_name);
        (*pr).description =
            libintl_dgettext(b"openconnect\x00" as *const u8 as
                                 *const libc::c_char, (*p).description);
        (*pr).flags = (*p).flags;
        p = p.offset(1);
        pr = pr.offset(1)
    }
    return p.wrapping_offset_from(openconnect_protos.as_ptr()) as libc::c_long
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_free_supported_protocols(mut protos:
                                                                  *mut oc_vpn_proto) {
    free(protos as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_protocol(mut vpninfo:
                                                      *mut openconnect_info)
 -> *const libc::c_char {
    return (*(*vpninfo).proto).name;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_protocol(mut vpninfo:
                                                      *mut openconnect_info,
                                                  mut protocol:
                                                      *const libc::c_char)
 -> libc::c_int {
    let mut p: *const vpn_proto = 0 as *const vpn_proto;
    p = openconnect_protos.as_ptr();
    while !(*p).name.is_null() {
        if strcasecmp((*p).name, protocol) != 0 {
            p = p.offset(1)
        } else {
            (*vpninfo).proto = p;
            if (*p).udp_setup.is_none() { (*vpninfo).dtls_state = 2i32 }
            return 0i32
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
                                                                                 b"Unknown VPN protocol \'%s\'\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char),
                                                                protocol);
    }
    return -22i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_pass_tos(mut vpninfo:
                                                      *mut openconnect_info,
                                                  mut enable: libc::c_int) {
    (*vpninfo).dtls_pass_tos = enable;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_loglevel(mut vpninfo:
                                                      *mut openconnect_info,
                                                  mut level: libc::c_int) {
    (*vpninfo).verbose = level;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_setup_dtls(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut attempt_period:
                                                    libc::c_int)
 -> libc::c_int {
    if (*(*vpninfo).proto).udp_setup.is_some() {
        return (*(*vpninfo).proto).udp_setup.expect("non-null function pointer")(vpninfo,
                                                                                 attempt_period)
    }
    if (*vpninfo).verbose >= 0i32 {
        (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                0i32,
                                                                libintl_dgettext(b"openconnect\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Built against SSL library with no Cisco DTLS support\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
    }
    return -22i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_obtain_cookie(mut vpninfo:
                                                       *mut openconnect_info)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*vpninfo).token_mode == OC_TOKEN_MODE_STOKEN as libc::c_int {
        ret = prepare_stoken(vpninfo);
        if ret != 0 { return ret }
    }
    return (*(*vpninfo).proto).obtain_cookie.expect("non-null function pointer")(vpninfo);
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_make_cstp_connection(mut vpninfo:
                                                              *mut openconnect_info)
 -> libc::c_int {
    return (*(*vpninfo).proto).tcp_connect.expect("non-null function pointer")(vpninfo);
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_reported_os(mut vpninfo:
                                                         *mut openconnect_info,
                                                     mut os:
                                                         *const libc::c_char)
 -> libc::c_int {
    if os.is_null() {
        os = b"mac-intel\x00" as *const u8 as *const libc::c_char
    }
    if strcmp(os, b"mac-intel\x00" as *const u8 as *const libc::c_char) == 0 {
        (*vpninfo).csd_xmltag =
            b"csdMac\x00" as *const u8 as *const libc::c_char
    } else if strcmp(os, b"linux\x00" as *const u8 as *const libc::c_char) ==
                  0 ||
                  strcmp(os,
                         b"linux-64\x00" as *const u8 as *const libc::c_char)
                      == 0 {
        (*vpninfo).csd_xmltag =
            b"csdLinux\x00" as *const u8 as *const libc::c_char
    } else if strcmp(os, b"android\x00" as *const u8 as *const libc::c_char)
                  == 0 ||
                  strcmp(os,
                         b"apple-ios\x00" as *const u8 as *const libc::c_char)
                      == 0 {
        (*vpninfo).csd_xmltag =
            b"csdLinux\x00" as *const u8 as *const libc::c_char;
        (*vpninfo).csd_nostub = 1i32
    } else if strcmp(os, b"win\x00" as *const u8 as *const libc::c_char) == 0
     {
        (*vpninfo).csd_xmltag = b"csd\x00" as *const u8 as *const libc::c_char
    } else { return -22i32 }
    if (*vpninfo).platname != os as *mut libc::c_char {
        free((*vpninfo).platname as *mut libc::c_void);
        if !os.is_null() {
            (*vpninfo).platname = strdup(os);
            if (*vpninfo).platname.is_null() { return -12i32 }
        } else { (*vpninfo).platname = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_mobile_info(mut vpninfo:
                                                         *mut openconnect_info,
                                                     mut mobile_platform_version:
                                                         *const libc::c_char,
                                                     mut mobile_device_type:
                                                         *const libc::c_char,
                                                     mut mobile_device_uniqueid:
                                                         *const libc::c_char)
 -> libc::c_int {
    if (*vpninfo).mobile_platform_version !=
           mobile_platform_version as *mut libc::c_char {
        free((*vpninfo).mobile_platform_version as *mut libc::c_void);
        if !mobile_platform_version.is_null() {
            (*vpninfo).mobile_platform_version =
                strdup(mobile_platform_version);
            if (*vpninfo).mobile_platform_version.is_null() { return -12i32 }
        } else { (*vpninfo).mobile_platform_version = 0 as *mut libc::c_char }
    }
    if (*vpninfo).mobile_device_type !=
           mobile_device_type as *mut libc::c_char {
        free((*vpninfo).mobile_device_type as *mut libc::c_void);
        if !mobile_device_type.is_null() {
            (*vpninfo).mobile_device_type = strdup(mobile_device_type);
            if (*vpninfo).mobile_device_type.is_null() { return -12i32 }
        } else { (*vpninfo).mobile_device_type = 0 as *mut libc::c_char }
    }
    if (*vpninfo).mobile_device_uniqueid !=
           mobile_device_uniqueid as *mut libc::c_char {
        free((*vpninfo).mobile_device_uniqueid as *mut libc::c_void);
        if !mobile_device_uniqueid.is_null() {
            (*vpninfo).mobile_device_uniqueid =
                strdup(mobile_device_uniqueid);
            if (*vpninfo).mobile_device_uniqueid.is_null() { return -12i32 }
        } else { (*vpninfo).mobile_device_uniqueid = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_version_string(mut vpninfo:
                                                            *mut openconnect_info,
                                                        mut version_string:
                                                            *const libc::c_char)
 -> libc::c_int {
    if (*vpninfo).version_string != version_string as *mut libc::c_char {
        free((*vpninfo).version_string as *mut libc::c_void);
        if !version_string.is_null() {
            (*vpninfo).version_string = strdup(version_string);
            if (*vpninfo).version_string.is_null() { return -12i32 }
        } else { (*vpninfo).version_string = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn free_optlist(mut opt: *mut oc_vpn_option) {
    let mut next: *mut oc_vpn_option = 0 as *mut oc_vpn_option;
    while !opt.is_null() {
        next = (*opt).next;
        free((*opt).option as *mut libc::c_void);
        free((*opt).value as *mut libc::c_void);
        free(opt as *mut libc::c_void);
        opt = next
    };
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_vpninfo_free(mut vpninfo:
                                                      *mut openconnect_info) {
    openconnect_close_https(vpninfo, 1i32);
    if (*(*vpninfo).proto).udp_shutdown.is_some() {
        (*(*vpninfo).proto).udp_shutdown.expect("non-null function pointer")(vpninfo);
    }
    if (*vpninfo).tncc_fd != -1i32 { close((*vpninfo).tncc_fd); }
    if (*vpninfo).cmd_fd_write != -1i32 {
        close((*vpninfo).cmd_fd);
        close((*vpninfo).cmd_fd_write);
    }
    if (*vpninfo).ic_utf8_to_legacy != -1i32 as iconv_t {
        iconv_close((*vpninfo).ic_utf8_to_legacy);
    }
    if (*vpninfo).ic_legacy_to_utf8 != -1i32 as iconv_t {
        iconv_close((*vpninfo).ic_legacy_to_utf8);
    }
    free((*vpninfo).peer_addr as *mut libc::c_void);
    free((*vpninfo).ip_info.gateway_addr as *mut libc::c_void);
    free_optlist((*vpninfo).csd_env);
    free_optlist((*vpninfo).script_env);
    free_optlist((*vpninfo).cookies);
    free_optlist((*vpninfo).cstp_options);
    free_optlist((*vpninfo).dtls_options);
    free_split_routes(vpninfo);
    free((*vpninfo).hostname as *mut libc::c_void);
    free((*vpninfo).unique_hostname as *mut libc::c_void);
    free((*vpninfo).urlpath as *mut libc::c_void);
    free((*vpninfo).redirect_url as *mut libc::c_void);
    free_pass(&mut (*vpninfo).cookie);
    free((*vpninfo).proxy_type as *mut libc::c_void);
    free((*vpninfo).proxy as *mut libc::c_void);
    free((*vpninfo).proxy_user as *mut libc::c_void);
    free_pass(&mut (*vpninfo).proxy_pass);
    free_pass(&mut (*vpninfo).cert_password);
    free((*vpninfo).vpnc_script as *mut libc::c_void);
    free((*vpninfo).cafile as *mut libc::c_void);
    free((*vpninfo).ifname as *mut libc::c_void);
    free((*vpninfo).dtls_cipher as *mut libc::c_void);
    free((*vpninfo).peer_cert_hash as *mut libc::c_void);
    free((*vpninfo).dtls_addr as *mut libc::c_void);
    if !(*vpninfo).csd_scriptname.is_null() {
        unlink((*vpninfo).csd_scriptname);
        free((*vpninfo).csd_scriptname as *mut libc::c_void);
    }
    free((*vpninfo).mobile_platform_version as *mut libc::c_void);
    free((*vpninfo).mobile_device_type as *mut libc::c_void);
    free((*vpninfo).mobile_device_uniqueid as *mut libc::c_void);
    free((*vpninfo).csd_token as *mut libc::c_void);
    free((*vpninfo).csd_ticket as *mut libc::c_void);
    free((*vpninfo).csd_stuburl as *mut libc::c_void);
    free((*vpninfo).csd_starturl as *mut libc::c_void);
    free((*vpninfo).csd_waiturl as *mut libc::c_void);
    free((*vpninfo).csd_preurl as *mut libc::c_void);
    free((*vpninfo).platname as *mut libc::c_void);
    if !(*vpninfo).opaque_srvdata.is_null() {
        xmlFreeNode((*vpninfo).opaque_srvdata);
    }
    free((*vpninfo).profile_url as *mut libc::c_void);
    free((*vpninfo).profile_sha1 as *mut libc::c_void);
    /* These are const in openconnect itself, but for consistency of
	   the library API we do take ownership of the strings we're given,
	   and thus we have to free them too. */
    if (*vpninfo).cert != (*vpninfo).sslkey {
        free((*vpninfo).sslkey as *mut libc::c_void);
    }
    free((*vpninfo).cert as *mut libc::c_void);
    if !(*vpninfo).peer_cert.is_null() {
        X509_free((*vpninfo).peer_cert as *mut X509);
        (*vpninfo).peer_cert = 0 as *mut libc::c_void
    }
    while !(*vpninfo).pin_cache.is_null() {
        let mut cache: *mut pin_cache = (*vpninfo).pin_cache;
        free((*cache).token as *mut libc::c_void);
        memset((*cache).pin as *mut libc::c_void, 0x5ai32,
               strlen((*cache).pin));
        free((*cache).pin as *mut libc::c_void);
        (*vpninfo).pin_cache = (*cache).next;
        free(cache as *mut libc::c_void);
    }
    free((*vpninfo).localname as *mut libc::c_void);
    free((*vpninfo).useragent as *mut libc::c_void);
    free((*vpninfo).authgroup as *mut libc::c_void);
    if !(*vpninfo).stoken_pin.is_null() {
        free_pass(&mut (*vpninfo).stoken_pin);
    }
    if !(*vpninfo).stoken_ctx.is_null() {
        stoken_destroy((*vpninfo).stoken_ctx);
    }
    if !(*vpninfo).oath_secret.is_null() {
        /* HAVE_LIBPSKC */
        free_pass(&mut (*vpninfo).oath_secret);
    }
    release_pcsc_ctx(vpninfo);
    /* These check strm->state so they are safe to call multiple times */
    inflateEnd(&mut (*vpninfo).inflate_strm);
    deflateEnd(&mut (*vpninfo).deflate_strm);
    free((*vpninfo).deflate_pkt as *mut libc::c_void);
    free((*vpninfo).tun_pkt as *mut libc::c_void);
    free((*vpninfo).dtls_pkt as *mut libc::c_void);
    free((*vpninfo).cstp_pkt as *mut libc::c_void);
    free(vpninfo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_hostname(mut vpninfo:
                                                      *mut openconnect_info)
 -> *const libc::c_char {
    return if !(*vpninfo).unique_hostname.is_null() {
               (*vpninfo).unique_hostname
           } else { (*vpninfo).hostname };
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_dnsname(mut vpninfo:
                                                     *mut openconnect_info)
 -> *const libc::c_char {
    return (*vpninfo).hostname;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_hostname(mut vpninfo:
                                                      *mut openconnect_info,
                                                  mut hostname:
                                                      *const libc::c_char)
 -> libc::c_int {
    if !hostname.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, hostname) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 25],
                                                                                              &[libc::c_char; 25]>(b"openconnect_set_hostname\x00")).as_ptr(),
                                                                    b"hostname\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    if (*vpninfo).hostname != hostname as *mut libc::c_char {
        free((*vpninfo).hostname as *mut libc::c_void);
        if !hostname.is_null() {
            (*vpninfo).hostname = strdup(hostname);
            if (*vpninfo).hostname.is_null() { return -12i32 }
        } else { (*vpninfo).hostname = 0 as *mut libc::c_char }
    }
    free((*vpninfo).unique_hostname as *mut libc::c_void);
    (*vpninfo).unique_hostname = 0 as *mut libc::c_char;
    free((*vpninfo).peer_addr as *mut libc::c_void);
    (*vpninfo).peer_addr = 0 as *mut sockaddr;
    free((*vpninfo).ip_info.gateway_addr as *mut libc::c_void);
    (*vpninfo).ip_info.gateway_addr = 0 as *mut libc::c_char;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_urlpath(mut vpninfo:
                                                     *mut openconnect_info)
 -> *mut libc::c_char {
    return (*vpninfo).urlpath;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_urlpath(mut vpninfo:
                                                     *mut openconnect_info,
                                                 mut urlpath:
                                                     *const libc::c_char)
 -> libc::c_int {
    if !urlpath.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, urlpath) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 24],
                                                                                              &[libc::c_char; 24]>(b"openconnect_set_urlpath\x00")).as_ptr(),
                                                                    b"urlpath\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    if (*vpninfo).urlpath != urlpath as *mut libc::c_char {
        free((*vpninfo).urlpath as *mut libc::c_void);
        if !urlpath.is_null() {
            (*vpninfo).urlpath = strdup(urlpath);
            if (*vpninfo).urlpath.is_null() { return -12i32 }
        } else { (*vpninfo).urlpath = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_localname(mut vpninfo:
                                                       *mut openconnect_info,
                                                   mut localname:
                                                       *const libc::c_char)
 -> libc::c_int {
    if !localname.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, localname) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 26],
                                                                                              &[libc::c_char; 26]>(b"openconnect_set_localname\x00")).as_ptr(),
                                                                    b"localname\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    if (*vpninfo).localname != localname as *mut libc::c_char {
        free((*vpninfo).localname as *mut libc::c_void);
        if !localname.is_null() {
            (*vpninfo).localname = strdup(localname);
            if (*vpninfo).localname.is_null() { return -12i32 }
        } else { (*vpninfo).localname = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_xmlsha1(mut vpninfo:
                                                     *mut openconnect_info,
                                                 mut xmlsha1:
                                                     *const libc::c_char,
                                                 mut size: libc::c_int) {
    if size as libc::c_ulong !=
           ::std::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong {
        return
    }
    memcpy(&mut (*vpninfo).xmlsha1 as *mut [libc::c_char; 41] as
               *mut libc::c_void, xmlsha1 as *const libc::c_void,
           size as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_disable_ipv6(mut vpninfo:
                                                      *mut openconnect_info) {
    (*vpninfo).disable_ipv6 = 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_cafile(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut cafile:
                                                    *const libc::c_char)
 -> libc::c_int {
    if !cafile.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, cafile) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 23],
                                                                                              &[libc::c_char; 23]>(b"openconnect_set_cafile\x00")).as_ptr(),
                                                                    b"cafile\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    if (*vpninfo).cafile != cafile as *mut libc::c_char {
        free((*vpninfo).cafile as *mut libc::c_void);
        if !cafile.is_null() {
            (*vpninfo).cafile = strdup(cafile);
            if (*vpninfo).cafile.is_null() { return -12i32 }
        } else { (*vpninfo).cafile = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_system_trust(mut vpninfo:
                                                          *mut openconnect_info,
                                                      mut val: libc::c_uint) {
    (*vpninfo).no_system_trust = (val == 0) as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_ifname(mut vpninfo:
                                                    *mut openconnect_info)
 -> *const libc::c_char {
    return (*vpninfo).ifname;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_reqmtu(mut vpninfo:
                                                    *mut openconnect_info,
                                                mut reqmtu: libc::c_int) {
    (*vpninfo).reqmtu = reqmtu;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_dpd(mut vpninfo:
                                                 *mut openconnect_info,
                                             mut min_seconds: libc::c_int) {
    /* Make sure (ka->dpd / 2), our computed midway point, isn't 0 */
    if min_seconds == 0 || min_seconds >= 2i32 {
        (*vpninfo).ssl_times.dpd = min_seconds;
        (*vpninfo).dtls_times.dpd = (*vpninfo).ssl_times.dpd
    } else if min_seconds == 1i32 {
        (*vpninfo).ssl_times.dpd = 2i32;
        (*vpninfo).dtls_times.dpd = (*vpninfo).ssl_times.dpd
    };
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_idle_timeout(mut vpninfo:
                                                          *mut openconnect_info)
 -> libc::c_int {
    return (*vpninfo).idle_timeout;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_ip_info(mut vpninfo:
                                                     *mut openconnect_info,
                                                 mut info:
                                                     *mut *const oc_ip_info,
                                                 mut cstp_options:
                                                     *mut *const oc_vpn_option,
                                                 mut dtls_options:
                                                     *mut *const oc_vpn_option)
 -> libc::c_int {
    if !info.is_null() { *info = &mut (*vpninfo).ip_info }
    if !cstp_options.is_null() { *cstp_options = (*vpninfo).cstp_options }
    if !dtls_options.is_null() { *dtls_options = (*vpninfo).dtls_options }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_setup_csd(mut vpninfo:
                                                   *mut openconnect_info,
                                               mut uid: uid_t,
                                               mut silent: libc::c_int,
                                               mut wrapper:
                                                   *const libc::c_char)
 -> libc::c_int {
    (*vpninfo).uid_csd = uid;
    (*vpninfo).uid_csd_given = if silent != 0 { 2i32 } else { 1i32 };
    if (*vpninfo).csd_wrapper != wrapper as *mut libc::c_char {
        free((*vpninfo).csd_wrapper as *mut libc::c_void);
        if !wrapper.is_null() {
            (*vpninfo).csd_wrapper = strdup(wrapper);
            if (*vpninfo).csd_wrapper.is_null() { return -12i32 }
        } else { (*vpninfo).csd_wrapper = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_xmlpost(mut vpninfo:
                                                     *mut openconnect_info,
                                                 mut enable: libc::c_int) {
    (*vpninfo).xmlpost = enable;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_client_cert(mut vpninfo:
                                                         *mut openconnect_info,
                                                     mut cert:
                                                         *const libc::c_char,
                                                     mut sslkey:
                                                         *const libc::c_char)
 -> libc::c_int {
    if !cert.is_null() && buf_append_utf16le(0 as *mut oc_text_buf, cert) != 0
       {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 28],
                                                                                              &[libc::c_char; 28]>(b"openconnect_set_client_cert\x00")).as_ptr(),
                                                                    b"cert\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    if !sslkey.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, sslkey) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 28],
                                                                                              &[libc::c_char; 28]>(b"openconnect_set_client_cert\x00")).as_ptr(),
                                                                    b"sslkey\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    /* Avoid freeing it twice if it's the same */
    if (*vpninfo).sslkey == (*vpninfo).cert {
        (*vpninfo).sslkey = 0 as *mut libc::c_char
    }
    if (*vpninfo).cert != cert as *mut libc::c_char {
        free((*vpninfo).cert as *mut libc::c_void);
        if !cert.is_null() {
            (*vpninfo).cert = strdup(cert);
            if (*vpninfo).cert.is_null() { return -12i32 }
        } else { (*vpninfo).cert = 0 as *mut libc::c_char }
    }
    if !sslkey.is_null() {
        if (*vpninfo).sslkey != sslkey as *mut libc::c_char {
            free((*vpninfo).sslkey as *mut libc::c_void);
            if !sslkey.is_null() {
                (*vpninfo).sslkey = strdup(sslkey);
                if (*vpninfo).sslkey.is_null() { return -12i32 }
            } else { (*vpninfo).sslkey = 0 as *mut libc::c_char }
        }
    } else { (*vpninfo).sslkey = (*vpninfo).cert }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_port(mut vpninfo:
                                                  *mut openconnect_info)
 -> libc::c_int {
    return (*vpninfo).port;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_cookie(mut vpninfo:
                                                    *mut openconnect_info)
 -> *const libc::c_char {
    return (*vpninfo).cookie;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_clear_cookie(mut vpninfo:
                                                      *mut openconnect_info) {
    if !(*vpninfo).cookie.is_null() {
        memset((*vpninfo).cookie as *mut libc::c_void, 0i32,
               strlen((*vpninfo).cookie));
    };
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_reset_ssl(mut vpninfo:
                                                   *mut openconnect_info) {
    (*vpninfo).got_cancel_cmd = 0i32;
    openconnect_close_https(vpninfo, 0i32);
    free((*vpninfo).peer_addr as *mut libc::c_void);
    (*vpninfo).peer_addr = 0 as *mut sockaddr;
    (*vpninfo).dtls_tos_optname = 0i32;
    free((*vpninfo).ip_info.gateway_addr as *mut libc::c_void);
    (*vpninfo).ip_info.gateway_addr = 0 as *mut libc::c_char;
    openconnect_clear_cookies(vpninfo);
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_parse_url(mut vpninfo:
                                                   *mut openconnect_info,
                                               mut url: *const libc::c_char)
 -> libc::c_int {
    let mut scheme: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    if !url.is_null() && buf_append_utf16le(0 as *mut oc_text_buf, url) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 22],
                                                                                              &[libc::c_char; 22]>(b"openconnect_parse_url\x00")).as_ptr(),
                                                                    b"url\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    openconnect_set_hostname(vpninfo, 0 as *const libc::c_char);
    free((*vpninfo).urlpath as *mut libc::c_void);
    (*vpninfo).urlpath = 0 as *mut libc::c_char;
    ret =
        internal_parse_url(url, &mut scheme, &mut (*vpninfo).hostname,
                           &mut (*vpninfo).port, &mut (*vpninfo).urlpath,
                           443i32);
    if ret != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Failed to parse server URL \'%s\'\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    url);
        }
        return ret
    }
    if !scheme.is_null() &&
           strcmp(scheme, b"https\x00" as *const u8 as *const libc::c_char) !=
               0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"Only https:// permitted for server URL\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        ret = -22i32
    }
    free(scheme as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_cert_expiry_warning(mut vpninfo:
                                                                 *mut openconnect_info,
                                                             mut seconds:
                                                                 libc::c_int) {
    (*vpninfo).cert_expire_warning = seconds;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_key_password(mut vpninfo:
                                                          *mut openconnect_info,
                                                      mut pass:
                                                          *const libc::c_char)
 -> libc::c_int {
    if (*vpninfo).cert_password != pass as *mut libc::c_char {
        free((*vpninfo).cert_password as *mut libc::c_void);
        if !pass.is_null() {
            (*vpninfo).cert_password = strdup(pass);
            if (*vpninfo).cert_password.is_null() { return -12i32 }
        } else { (*vpninfo).cert_password = 0 as *mut libc::c_char }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_pfs(mut vpninfo:
                                                 *mut openconnect_info,
                                             mut val: libc::c_uint) {
    (*vpninfo).pfs = val;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_cancel_fd(mut vpninfo:
                                                       *mut openconnect_info,
                                                   mut fd: libc::c_int) {
    (*vpninfo).cmd_fd = fd;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_setup_cmd_pipe(mut vpninfo:
                                                        *mut openconnect_info)
 -> libc::c_int {
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    if pipe(pipefd.as_mut_ptr()) < 0i32 { return -5i32 }
    if set_sock_nonblock(pipefd[0]) != 0 || set_sock_nonblock(pipefd[1]) != 0
       {
        close(pipefd[0]);
        close(pipefd[1]);
        return -5i32
    }
    (*vpninfo).cmd_fd = pipefd[0];
    (*vpninfo).cmd_fd_write = pipefd[1];
    return (*vpninfo).cmd_fd_write;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_version() -> *const libc::c_char {
    return openconnect_version_str;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_has_pkcs11_support() -> libc::c_int {
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_has_tss_blob_support() -> libc::c_int {
    let mut e: *mut ENGINE = 0 as *mut ENGINE;
    ENGINE_load_builtin_engines();
    e = ENGINE_by_id(b"tpm\x00" as *const u8 as *const libc::c_char);
    if !e.is_null() { ENGINE_free(e); return 1i32 }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_has_tss2_blob_support() -> libc::c_int {
    let mut e: *mut ENGINE = 0 as *mut ENGINE;
    ENGINE_load_builtin_engines();
    e = ENGINE_by_id(b"tpm2\x00" as *const u8 as *const libc::c_char);
    if !e.is_null() { ENGINE_free(e); return 1i32 }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_has_stoken_support() -> libc::c_int {
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_has_oath_support() -> libc::c_int {
    return 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_has_yubioath_support() -> libc::c_int {
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_has_system_key_support() -> libc::c_int {
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_token_callbacks(mut vpninfo:
                                                             *mut openconnect_info,
                                                         mut tokdata:
                                                             *mut libc::c_void,
                                                         mut lock:
                                                             openconnect_lock_token_vfn,
                                                         mut unlock:
                                                             openconnect_unlock_token_vfn)
 -> libc::c_int {
    (*vpninfo).lock_token = lock;
    (*vpninfo).unlock_token = unlock;
    (*vpninfo).tok_cbdata = tokdata;
    return 0i32;
}
/*
 * Enable software token generation.
 *
 * If token_mode is OC_TOKEN_MODE_STOKEN and token_str is NULL,
 * read the token data from ~/.stokenrc.
 *
 * Return value:
 *  = -EILSEQ, if token_str is not valid UTF-8
 *  = -EOPNOTSUPP, if the underlying library (libstoken, liboath) is not
 *                 available or an invalid token_mode was provided
 *  = -EINVAL, if the token string is invalid (token_str was provided)
 *  = -ENOENT, if token_mode is OC_TOKEN_MODE_STOKEN and ~/.stokenrc is
 *             missing (token_str was NULL)
 *  = -EIO, for other failures in the underlying library (libstoken, liboath)
 *  = 0, on success
 */
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_token_mode(mut vpninfo:
                                                        *mut openconnect_info,
                                                    mut token_mode:
                                                        oc_token_mode_t,
                                                    mut token_str:
                                                        *const libc::c_char)
 -> libc::c_int {
    (*vpninfo).token_mode = OC_TOKEN_MODE_NONE as libc::c_int;
    if !token_str.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, token_str) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 27],
                                                                                              &[libc::c_char; 27]>(b"openconnect_set_token_mode\x00")).as_ptr(),
                                                                    b"token_str\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    match token_mode as libc::c_uint {
        0 => { return 0i32 }
        1 => { return set_libstoken_mode(vpninfo, token_str) }
        2 => { return set_totp_mode(vpninfo, token_str) }
        3 => { return set_hotp_mode(vpninfo, token_str) }
        4 => { return set_yubikey_mode(vpninfo, token_str) }
        _ => { return -102i32 }
    };
}
/*
 * Enable libstoken token generation if use_stoken == 1.
 *
 * If token_str is not NULL, try to parse the string.  Otherwise, try to read
 * the token data from ~/.stokenrc
 *
 * DEPRECATED: use openconnect_set_stoken_mode() instead.
 *
 * Return value:
 *  = -EILSEQ, if token_str is not valid UTF-8
 *  = -EOPNOTSUPP, if libstoken is not available
 *  = -EINVAL, if the token string is invalid (token_str was provided)
 *  = -ENOENT, if ~/.stokenrc is missing (token_str was NULL)
 *  = -EIO, for other libstoken failures
 *  = 0, on success
 */
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_stoken_mode(mut vpninfo:
                                                         *mut openconnect_info,
                                                     mut use_stoken:
                                                         libc::c_int,
                                                     mut token_str:
                                                         *const libc::c_char)
 -> libc::c_int {
    let mut token_mode: oc_token_mode_t = OC_TOKEN_MODE_NONE;
    if use_stoken != 0 { token_mode = OC_TOKEN_MODE_STOKEN }
    return openconnect_set_token_mode(vpninfo, token_mode, token_str);
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_protect_socket_handler(mut vpninfo:
                                                                    *mut openconnect_info,
                                                                mut protect_socket:
                                                                    openconnect_protect_socket_vfn) {
    (*vpninfo).protect_socket = protect_socket;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_override_getaddrinfo(mut vpninfo:
                                                              *mut openconnect_info,
                                                          mut gai_fn:
                                                              openconnect_getaddrinfo_vfn) {
    (*vpninfo).getaddrinfo_override = gai_fn;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_setup_tun_handler(mut vpninfo:
                                                               *mut openconnect_info,
                                                           mut setup_tun:
                                                               openconnect_setup_tun_vfn) {
    (*vpninfo).setup_tun = setup_tun;
}
/* SSL certificate capabilities. openconnect_has_pkcs11_support() means that we
   can accept PKCS#11 URLs in place of filenames, for the certificate and key. */
/* The OpenSSL TPM ENGINE stores keys in a PEM file labelled with the string
   -----BEGIN TSS KEY BLOB-----. */
/* Software token capabilities. */
/* Query and select from among supported protocols */
/* Callback for configuring the interface after MTU detection finishes. */
/* Callback for indicating that a TCP reconnection succeeded. */
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_reconnected_handler(mut vpninfo:
                                                                 *mut openconnect_info,
                                                             mut reconnected:
                                                                 openconnect_reconnected_vfn) {
    (*vpninfo).reconnected = reconnected;
}
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
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_stats_handler(mut vpninfo:
                                                           *mut openconnect_info,
                                                       mut stats_handler:
                                                           openconnect_stats_vfn) {
    (*vpninfo).stats_handler = stats_handler;
}
/* Set up a traditional OS-based tunnel device, optionally specified in 'ifname'. */
#[no_mangle]
pub unsafe extern "C" fn openconnect_setup_tun_device(mut vpninfo:
                                                          *mut openconnect_info,
                                                      mut vpnc_script:
                                                          *const libc::c_char,
                                                      mut ifname:
                                                          *const libc::c_char)
 -> libc::c_int {
    let mut tun_fd: intptr_t = 0;
    let mut legacy_ifname: *mut libc::c_char = 0 as *mut libc::c_char;
    if !vpnc_script.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, vpnc_script) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 29],
                                                                                              &[libc::c_char; 29]>(b"openconnect_setup_tun_device\x00")).as_ptr(),
                                                                    b"vpnc_script\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    if !ifname.is_null() &&
           buf_append_utf16le(0 as *mut oc_text_buf, ifname) != 0 {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"ERROR: %s() called with invalid UTF-8 for \'%s\' argument\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char),
                                                                    (*::std::mem::transmute::<&[u8; 29],
                                                                                              &[libc::c_char; 29]>(b"openconnect_setup_tun_device\x00")).as_ptr(),
                                                                    b"ifname\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
        }
        return -92i32
    }
    if (*vpninfo).vpnc_script != vpnc_script as *mut libc::c_char {
        free((*vpninfo).vpnc_script as *mut libc::c_void);
        if !vpnc_script.is_null() {
            (*vpninfo).vpnc_script = strdup(vpnc_script);
            if (*vpninfo).vpnc_script.is_null() { return -12i32 }
        } else { (*vpninfo).vpnc_script = 0 as *mut libc::c_char }
    }
    if (*vpninfo).ifname != ifname as *mut libc::c_char {
        free((*vpninfo).ifname as *mut libc::c_void);
        if !ifname.is_null() {
            (*vpninfo).ifname = strdup(ifname);
            if (*vpninfo).ifname.is_null() { return -12i32 }
        } else { (*vpninfo).ifname = 0 as *mut libc::c_char }
    }
    prepare_script_env(vpninfo);
    script_config_tun(vpninfo,
                      b"pre-init\x00" as *const u8 as *const libc::c_char);
    tun_fd = os_setup_tun(vpninfo);
    if tun_fd < 0i32 as libc::c_long { return tun_fd as libc::c_int }
    legacy_ifname = openconnect_utf8_to_legacy(vpninfo, (*vpninfo).ifname);
    script_setenv(vpninfo, b"TUNDEV\x00" as *const u8 as *const libc::c_char,
                  legacy_ifname, 0i32, 0i32);
    if legacy_ifname != (*vpninfo).ifname {
        free(legacy_ifname as *mut libc::c_void);
    }
    script_config_tun(vpninfo,
                      b"connect\x00" as *const u8 as *const libc::c_char);
    return openconnect_setup_tun_fd(vpninfo, tun_fd as libc::c_int);
}
static mut compr_name_map: [*const libc::c_char; 9] =
    [0 as *const libc::c_char,
     b"Deflate\x00" as *const u8 as *const libc::c_char,
     b"LZS\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char,
     b"LZ4\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char,
     0 as *const libc::c_char, 0 as *const libc::c_char,
     b"LZO\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_cstp_compression(mut vpninfo:
                                                              *mut openconnect_info)
 -> *const libc::c_char {
    if (*vpninfo).cstp_compr <= 0i32 || (*vpninfo).cstp_compr > 1i32 << 3i32 {
        return 0 as *const libc::c_char
    }
    return compr_name_map[(*vpninfo).cstp_compr as usize];
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_dtls_compression(mut vpninfo:
                                                              *mut openconnect_info)
 -> *const libc::c_char {
    if (*vpninfo).dtls_compr <= 0i32 || (*vpninfo).dtls_compr > 1i32 << 3i32 {
        return 0 as *const libc::c_char
    }
    return compr_name_map[(*vpninfo).dtls_compr as usize];
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_dtls_cipher(mut vpninfo:
                                                         *mut openconnect_info)
 -> *const libc::c_char {
    if !(*vpninfo).dtls_ssl.is_null() {
        return SSL_CIPHER_get_name(SSL_get_current_cipher((*vpninfo).dtls_ssl))
    } else { return 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_csd_environ(mut vpninfo:
                                                         *mut openconnect_info,
                                                     mut name:
                                                         *const libc::c_char,
                                                     mut value:
                                                         *const libc::c_char)
 -> libc::c_int {
    let mut p: *mut oc_vpn_option = 0 as *mut oc_vpn_option;
    if name.is_null() {
        free_optlist((*vpninfo).csd_env);
        (*vpninfo).csd_env = 0 as *mut oc_vpn_option;
        return 0i32
    }
    p = (*vpninfo).csd_env;
    while !p.is_null() {
        if strcmp(name, (*p).option) == 0 {
            let mut valdup: *mut libc::c_char = strdup(value);
            if valdup.is_null() { return -12i32 }
            free((*p).value as *mut libc::c_void);
            (*p).value = valdup;
            return 0i32
        }
        p = (*p).next
    }
    p =
        malloc(::std::mem::size_of::<oc_vpn_option>() as libc::c_ulong) as
            *mut oc_vpn_option;
    if p.is_null() { return -12i32 }
    (*p).option = strdup(name);
    if (*p).option.is_null() { free(p as *mut libc::c_void); return -12i32 }
    (*p).value = strdup(value);
    if (*p).value.is_null() {
        free((*p).option as *mut libc::c_void);
        free(p as *mut libc::c_void);
        return -12i32
    }
    (*p).next = (*vpninfo).csd_env;
    (*vpninfo).csd_env = p;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_check_peer_cert_hash(mut vpninfo:
                                                              *mut openconnect_info,
                                                          mut old_hash:
                                                              *const libc::c_char)
 -> libc::c_int {
    let mut fingerprint: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut min_match_len: libc::c_uint = 0;
    let mut real_min_match_len: libc::c_uint = 4i32 as libc::c_uint;
    let mut old_len: libc::c_uint = 0;
    let mut fingerprint_len: libc::c_uint = 0;
    let mut ret: libc::c_int = 0i32;
    if !strchr(old_hash, ':' as i32).is_null() {
        if strncmp(old_hash, b"sha1:\x00" as *const u8 as *const libc::c_char,
                   5i32 as libc::c_ulong) == 0i32 {
            fingerprint =
                openconnect_bin2hex(b"sha1:\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*vpninfo).peer_cert_sha1_raw.as_mut_ptr(),
                                    ::std::mem::size_of::<[uint8_t; 20]>() as
                                        libc::c_ulong as libc::c_uint);
            min_match_len =
                (real_min_match_len as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 6]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)
                    as libc::c_uint
        } else if strncmp(old_hash,
                          b"sha256:\x00" as *const u8 as *const libc::c_char,
                          7i32 as libc::c_ulong) == 0i32 {
            fingerprint =
                openconnect_bin2hex(b"sha256:\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*vpninfo).peer_cert_sha256_raw.as_mut_ptr(),
                                    ::std::mem::size_of::<[uint8_t; 32]>() as
                                        libc::c_ulong as libc::c_uint);
            min_match_len =
                (real_min_match_len as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 8]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)
                    as libc::c_uint
        } else if strncmp(old_hash,
                          b"pin-sha256:\x00" as *const u8 as
                              *const libc::c_char, 11i32 as libc::c_ulong) ==
                      0i32 {
            fingerprint =
                openconnect_bin2base64(b"pin-sha256:\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*vpninfo).peer_cert_sha256_raw.as_mut_ptr(),
                                       ::std::mem::size_of::<[uint8_t; 32]>()
                                           as libc::c_ulong as libc::c_uint);
            min_match_len =
                (real_min_match_len as
                     libc::c_ulong).wrapping_add(::std::mem::size_of::<[libc::c_char; 12]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)
                    as libc::c_uint
        } else {
            if (*vpninfo).verbose >= 0i32 {
                (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                        0i32,
                                                                        libintl_dgettext(b"openconnect\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"Unknown certificate hash: %s.\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char),
                                                                        old_hash);
            }
            return -5i32
        }
    } else {
        let mut cert: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut len: libc::c_int = 0;
        let mut sha1_bin: [libc::c_uchar; 20] = [0; 20];
        len = openconnect_get_peer_cert_DER(vpninfo, &mut cert);
        if len < 0i32 { return len }
        if openconnect_sha1(sha1_bin.as_mut_ptr(), cert as *mut libc::c_void,
                            len) != 0 {
            return -5i32
        }
        fingerprint =
            openconnect_bin2hex(0 as *const libc::c_char,
                                sha1_bin.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_uchar; 20]>()
                                    as libc::c_ulong as libc::c_uint);
        min_match_len = real_min_match_len
    }
    if fingerprint.is_null() { return -5i32 }
    old_len = strlen(old_hash) as libc::c_uint;
    fingerprint_len = strlen(fingerprint) as libc::c_uint;
    /* allow partial matches */
    if old_len < fingerprint_len {
        if strncasecmp(old_hash, fingerprint,
                       (if min_match_len > old_len {
                            min_match_len
                        } else { old_len }) as libc::c_ulong) != 0 {
            if old_len < min_match_len {
                if (*vpninfo).verbose >= 0i32 {
                    (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                            0i32,
                                                                            libintl_dgettext(b"openconnect\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"The size of the provided fingerprint is less than the minimum required (%u).\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char),
                                                                            real_min_match_len);
                }
            }
            ret = 1i32
        }
    } else if strcasecmp(old_hash, fingerprint) != 0 { ret = 1i32 }
    free(fingerprint as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_cstp_cipher(mut vpninfo:
                                                         *mut openconnect_info)
 -> *const libc::c_char {
    return (*vpninfo).cstp_cipher;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_get_peer_cert_hash(mut vpninfo:
                                                            *mut openconnect_info)
 -> *const libc::c_char {
    if (*vpninfo).peer_cert_hash.is_null() {
        (*vpninfo).peer_cert_hash =
            openconnect_bin2base64(b"pin-sha256:\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*vpninfo).peer_cert_sha256_raw.as_mut_ptr(),
                                   ::std::mem::size_of::<[uint8_t; 32]>() as
                                       libc::c_ulong as libc::c_uint)
    }
    return (*vpninfo).peer_cert_hash;
}
#[no_mangle]
pub unsafe extern "C" fn openconnect_set_compression_mode(mut vpninfo:
                                                              *mut openconnect_info,
                                                          mut mode:
                                                              oc_compression_mode_t)
 -> libc::c_int {
    match mode as libc::c_uint {
        0 => { (*vpninfo).req_compr = 0i32; return 0i32 }
        1 => {
            (*vpninfo).req_compr = 1i32 << 1i32 | 1i32 << 2i32 | 1i32 << 3i32;
            return 0i32
        }
        2 => {
            (*vpninfo).req_compr =
                1i32 << 1i32 | 1i32 << 2i32 | 1i32 << 3i32 | 1i32 << 0i32;
            return 0i32
        }
        _ => { return -22i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nuke_opt_values(mut opt: *mut oc_form_opt) {
    while !opt.is_null() {
        if (*opt).type_0 == 1i32 || (*opt).type_0 == 2i32 {
            free((*opt)._value as *mut libc::c_void);
            (*opt)._value = 0 as *mut libc::c_char
        }
        opt = (*opt).next
    };
}
/* GlobalProtect magic ping address (network-endian) */
/* For Juniper TNCC */
/* We need to give it back in the same form */
/* The SHA1 and SHA256 hashes of the peer's public key */
/* this value is cache for openconnect_get_peer_cert_hash */
/* Pointer to within cookies list */
/* OPENCONNECT_GNUTLS */
/* It may need to be larger than MTU */
/* For compressing outbound packets into */
/* The original packet associated with above */
/* Partially sent SSL packet */
/* Control packets to be sent on oNCP next */
/* For packetising incoming oNCP stream */
/* Packet buffers for receiving into */
/* How many bytes after payload for encryption (ESP HMAC) */
/* Local static configured values */
/* Returned by server */
/* Returned by server */
/* What we requested */
/* Accepted for CSTP */
/* Accepted for DTLS */
/* Attempt to redo DNS lookup on each CSTP reconnect */
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
/* auth-juniper.c */
/* oncp.c */
/* pulse.c */
/* auth-globalprotect.c */
/* gpst.c */
/* lzs.c */
/* ssl.c */
/* openssl-pkcs11.c */
/* esp.c */
/* {gnutls,openssl}-esp.c */
/* {gnutls,openssl}.c */
/* mainloop.c */
/* xml.c */
/* oath.c */
/* stoken.c */
/* yubikey.c */
/* auth.c */
/* auth-common.c */
/* http.c */
/* http-auth.c */
/* ntlm.c */
/* gssapi.c */
/* digest.c */
/* library.c */
#[no_mangle]
pub unsafe extern "C" fn process_auth_form(mut vpninfo: *mut openconnect_info,
                                           mut form: *mut oc_auth_form)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut grp: *mut oc_form_opt_select = (*form).authgroup_opt;
    let mut auth_choice: *mut oc_choice = 0 as *mut oc_choice;
    let mut opt: *mut oc_form_opt = 0 as *mut oc_form_opt;
    if (*vpninfo).process_auth_form.is_none() {
        if (*vpninfo).verbose >= 0i32 {
            (*vpninfo).progress.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                    0i32,
                                                                    libintl_dgettext(b"openconnect\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"No form handler; cannot authenticate.\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
        }
        return -1i32
    }
    loop  {
        auth_choice = 0 as *mut oc_choice;
        if !grp.is_null() && (*grp).nr_choices != 0 {
            /* Set group selection from authgroup */
            if !(*vpninfo).authgroup.is_null() {
                let mut i: libc::c_int = 0;
                i = 0i32;
                while i < (*grp).nr_choices {
                    if strcmp((**(*grp).choices.offset(i as isize)).name,
                              (*vpninfo).authgroup) == 0 {
                        (*form).authgroup_selection = i
                    }
                    i += 1
                }
            }
            auth_choice =
                *(*grp).choices.offset((*form).authgroup_selection as isize)
        }
        opt = (*form).opts;
        while !opt.is_null() {
            let mut second_auth: libc::c_int =
                ((*opt).flags & 0x8000i32 as libc::c_uint) as libc::c_int;
            (*opt).flags &= !0x1i32 as libc::c_uint;
            if !(auth_choice.is_null() ||
                     (*opt).type_0 != 1i32 && (*opt).type_0 != 2i32) {
                if (*auth_choice).noaaa != 0 ||
                       (*auth_choice).second_auth == 0 && second_auth != 0 {
                    (*opt).flags |= 0x1i32 as libc::c_uint
                } else if strcmp((*opt).name,
                                 b"secondary_username\x00" as *const u8 as
                                     *const libc::c_char) == 0 &&
                              second_auth != 0 {
                    if !(*auth_choice).secondary_username.is_null() {
                        free((*opt)._value as *mut libc::c_void);
                        (*opt)._value =
                            strdup((*auth_choice).secondary_username)
                    }
                    if (*auth_choice).secondary_username_editable == 0 {
                        (*opt).flags |= 0x1i32 as libc::c_uint
                    }
                }
            }
            opt = (*opt).next
        }
        ret =
            (*vpninfo).process_auth_form.expect("non-null function pointer")((*vpninfo).cbdata,
                                                                             form);
        if !(ret == 2i32 && !(*form).authgroup_opt.is_null() &&
                 !(*(*form).authgroup_opt).form._value.is_null()) {
            break ;
        }
        free((*vpninfo).authgroup as *mut libc::c_void);
        (*vpninfo).authgroup = strdup((*(*form).authgroup_opt).form._value);
        if !((*vpninfo).xmlpost == 0) { break ; }
    }
    if ret == 1i32 || ret < 0i32 { nuke_opt_values((*form).opts); }
    return ret;
}
