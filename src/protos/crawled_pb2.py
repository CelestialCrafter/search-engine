# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: protos_raw/crawled.proto
# Protobuf Python Version: 4.25.3
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x18protos_raw/crawled.proto\x12\x07\x63rawler\x1a\x1fgoogle/protobuf/timestamp.proto\"\x98\x01\n\x07\x43rawled\x12\x0b\n\x03url\x18\x01 \x01(\t\x12\x10\n\x08\x63hildren\x18\x02 \x03(\t\x12\x32\n\tcrawledAt\x18\x03 \x01(\x0b\x32\x1a.google.protobuf.TimestampH\x00\x88\x01\x01\x12\x0c\n\x04mime\x18\x04 \x01(\t\x12\x10\n\x08original\x18\x05 \x01(\x0c\x12\x0c\n\x04text\x18\x06 \x01(\x0c\x42\x0c\n\n_crawledAtB\tZ\x07/protosb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'protos_raw.crawled_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
  _globals['DESCRIPTOR']._options = None
  _globals['DESCRIPTOR']._serialized_options = b'Z\007/protos'
  _globals['_CRAWLED']._serialized_start=71
  _globals['_CRAWLED']._serialized_end=223
# @@protoc_insertion_point(module_scope)
