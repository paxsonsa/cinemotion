// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: indiemotion-proto/messages/acknowledge.proto

#ifndef GOOGLE_PROTOBUF_INCLUDED_indiemotion_2dproto_2fmessages_2facknowledge_2eproto
#define GOOGLE_PROTOBUF_INCLUDED_indiemotion_2dproto_2fmessages_2facknowledge_2eproto

#include <limits>
#include <string>

#include <google/protobuf/port_def.inc>
#if PROTOBUF_VERSION < 3018000
#error This file was generated by a newer version of protoc which is
#error incompatible with your Protocol Buffer headers. Please update
#error your headers.
#endif
#if 3018001 < PROTOBUF_MIN_PROTOC_VERSION
#error This file was generated by an older version of protoc which is
#error incompatible with your Protocol Buffer headers. Please
#error regenerate this file with a newer version of protoc.
#endif

#include <google/protobuf/port_undef.inc>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/arena.h>
#include <google/protobuf/arenastring.h>
#include <google/protobuf/generated_message_table_driven.h>
#include <google/protobuf/generated_message_util.h>
#include <google/protobuf/metadata_lite.h>
#include <google/protobuf/generated_message_reflection.h>
#include <google/protobuf/message.h>
#include <google/protobuf/repeated_field.h>  // IWYU pragma: export
#include <google/protobuf/extension_set.h>  // IWYU pragma: export
#include <google/protobuf/unknown_field_set.h>
// @@protoc_insertion_point(includes)
#include <google/protobuf/port_def.inc>
#define PROTOBUF_INTERNAL_EXPORT_indiemotion_2dproto_2fmessages_2facknowledge_2eproto
PROTOBUF_NAMESPACE_OPEN
namespace internal {
class AnyMetadata;
}  // namespace internal
PROTOBUF_NAMESPACE_CLOSE

// Internal implementation detail -- do not use these members.
struct TableStruct_indiemotion_2dproto_2fmessages_2facknowledge_2eproto {
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTableField entries[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::AuxiliaryParseTableField aux[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTable schema[1]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::FieldMetadata field_metadata[];
  static const ::PROTOBUF_NAMESPACE_ID::internal::SerializationTable serialization_table[];
  static const uint32_t offsets[];
};
extern const ::PROTOBUF_NAMESPACE_ID::internal::DescriptorTable descriptor_table_indiemotion_2dproto_2fmessages_2facknowledge_2eproto;
namespace indiemotion {
namespace messages {
class Acknowledge;
struct AcknowledgeDefaultTypeInternal;
extern AcknowledgeDefaultTypeInternal _Acknowledge_default_instance_;
}  // namespace messages
}  // namespace indiemotion
PROTOBUF_NAMESPACE_OPEN
template<> ::indiemotion::messages::Acknowledge* Arena::CreateMaybeMessage<::indiemotion::messages::Acknowledge>(Arena*);
PROTOBUF_NAMESPACE_CLOSE
namespace indiemotion {
namespace messages {

// ===================================================================

class Acknowledge final :
    public ::PROTOBUF_NAMESPACE_ID::Message /* @@protoc_insertion_point(class_definition:indiemotion.messages.Acknowledge) */ {
 public:
  inline Acknowledge() : Acknowledge(nullptr) {}
  ~Acknowledge() override;
  explicit constexpr Acknowledge(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized);

  Acknowledge(const Acknowledge& from);
  Acknowledge(Acknowledge&& from) noexcept
    : Acknowledge() {
    *this = ::std::move(from);
  }

  inline Acknowledge& operator=(const Acknowledge& from) {
    CopyFrom(from);
    return *this;
  }
  inline Acknowledge& operator=(Acknowledge&& from) noexcept {
    if (this == &from) return *this;
    if (GetOwningArena() == from.GetOwningArena()
  #ifdef PROTOBUF_FORCE_COPY_IN_MOVE
        && GetOwningArena() != nullptr
  #endif  // !PROTOBUF_FORCE_COPY_IN_MOVE
    ) {
      InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* descriptor() {
    return GetDescriptor();
  }
  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* GetDescriptor() {
    return default_instance().GetMetadata().descriptor;
  }
  static const ::PROTOBUF_NAMESPACE_ID::Reflection* GetReflection() {
    return default_instance().GetMetadata().reflection;
  }
  static const Acknowledge& default_instance() {
    return *internal_default_instance();
  }
  static inline const Acknowledge* internal_default_instance() {
    return reinterpret_cast<const Acknowledge*>(
               &_Acknowledge_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  friend void swap(Acknowledge& a, Acknowledge& b) {
    a.Swap(&b);
  }
  inline void Swap(Acknowledge* other) {
    if (other == this) return;
  #ifdef PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() != nullptr &&
        GetOwningArena() == other->GetOwningArena()) {
   #else  // PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() == other->GetOwningArena()) {
  #endif  // !PROTOBUF_FORCE_COPY_IN_SWAP
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(Acknowledge* other) {
    if (other == this) return;
    GOOGLE_DCHECK(GetOwningArena() == other->GetOwningArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  Acknowledge* New(::PROTOBUF_NAMESPACE_ID::Arena* arena = nullptr) const final {
    return CreateMaybeMessage<Acknowledge>(arena);
  }
  using ::PROTOBUF_NAMESPACE_ID::Message::CopyFrom;
  void CopyFrom(const Acknowledge& from);
  using ::PROTOBUF_NAMESPACE_ID::Message::MergeFrom;
  void MergeFrom(const Acknowledge& from);
  private:
  static void MergeImpl(::PROTOBUF_NAMESPACE_ID::Message* to, const ::PROTOBUF_NAMESPACE_ID::Message& from);
  public:
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  uint8_t* _InternalSerialize(
      uint8_t* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _cached_size_.Get(); }

  private:
  void SharedCtor();
  void SharedDtor();
  void SetCachedSize(int size) const final;
  void InternalSwap(Acknowledge* other);

  private:
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::PROTOBUF_NAMESPACE_ID::StringPiece FullMessageName() {
    return "indiemotion.messages.Acknowledge";
  }
  protected:
  explicit Acknowledge(::PROTOBUF_NAMESPACE_ID::Arena* arena,
                       bool is_message_owned = false);
  private:
  static void ArenaDtor(void* object);
  inline void RegisterArenaDtor(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  public:

  static const ClassData _class_data_;
  const ::PROTOBUF_NAMESPACE_ID::Message::ClassData*GetClassData() const final;

  ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadata() const final;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kMessageFieldNumber = 2,
    kOkFieldNumber = 1,
  };
  // optional string message = 2;
  bool has_message() const;
  private:
  bool _internal_has_message() const;
  public:
  void clear_message();
  const std::string& message() const;
  template <typename ArgT0 = const std::string&, typename... ArgT>
  void set_message(ArgT0&& arg0, ArgT... args);
  std::string* mutable_message();
  PROTOBUF_NODISCARD std::string* release_message();
  void set_allocated_message(std::string* message);
  private:
  const std::string& _internal_message() const;
  inline PROTOBUF_ALWAYS_INLINE void _internal_set_message(const std::string& value);
  std::string* _internal_mutable_message();
  public:

  // bool ok = 1;
  void clear_ok();
  bool ok() const;
  void set_ok(bool value);
  private:
  bool _internal_ok() const;
  void _internal_set_ok(bool value);
  public:

  // @@protoc_insertion_point(class_scope:indiemotion.messages.Acknowledge)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  ::PROTOBUF_NAMESPACE_ID::internal::HasBits<1> _has_bits_;
  mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr message_;
  bool ok_;
  friend struct ::TableStruct_indiemotion_2dproto_2fmessages_2facknowledge_2eproto;
};
// ===================================================================


// ===================================================================

#ifdef __GNUC__
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// Acknowledge

// bool ok = 1;
inline void Acknowledge::clear_ok() {
  ok_ = false;
}
inline bool Acknowledge::_internal_ok() const {
  return ok_;
}
inline bool Acknowledge::ok() const {
  // @@protoc_insertion_point(field_get:indiemotion.messages.Acknowledge.ok)
  return _internal_ok();
}
inline void Acknowledge::_internal_set_ok(bool value) {
  
  ok_ = value;
}
inline void Acknowledge::set_ok(bool value) {
  _internal_set_ok(value);
  // @@protoc_insertion_point(field_set:indiemotion.messages.Acknowledge.ok)
}

// optional string message = 2;
inline bool Acknowledge::_internal_has_message() const {
  bool value = (_has_bits_[0] & 0x00000001u) != 0;
  return value;
}
inline bool Acknowledge::has_message() const {
  return _internal_has_message();
}
inline void Acknowledge::clear_message() {
  message_.ClearToEmpty();
  _has_bits_[0] &= ~0x00000001u;
}
inline const std::string& Acknowledge::message() const {
  // @@protoc_insertion_point(field_get:indiemotion.messages.Acknowledge.message)
  return _internal_message();
}
template <typename ArgT0, typename... ArgT>
inline PROTOBUF_ALWAYS_INLINE
void Acknowledge::set_message(ArgT0&& arg0, ArgT... args) {
 _has_bits_[0] |= 0x00000001u;
 message_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, static_cast<ArgT0 &&>(arg0), args..., GetArenaForAllocation());
  // @@protoc_insertion_point(field_set:indiemotion.messages.Acknowledge.message)
}
inline std::string* Acknowledge::mutable_message() {
  std::string* _s = _internal_mutable_message();
  // @@protoc_insertion_point(field_mutable:indiemotion.messages.Acknowledge.message)
  return _s;
}
inline const std::string& Acknowledge::_internal_message() const {
  return message_.Get();
}
inline void Acknowledge::_internal_set_message(const std::string& value) {
  _has_bits_[0] |= 0x00000001u;
  message_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, value, GetArenaForAllocation());
}
inline std::string* Acknowledge::_internal_mutable_message() {
  _has_bits_[0] |= 0x00000001u;
  return message_.Mutable(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, GetArenaForAllocation());
}
inline std::string* Acknowledge::release_message() {
  // @@protoc_insertion_point(field_release:indiemotion.messages.Acknowledge.message)
  if (!_internal_has_message()) {
    return nullptr;
  }
  _has_bits_[0] &= ~0x00000001u;
  auto* p = message_.ReleaseNonDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArenaForAllocation());
#ifdef PROTOBUF_FORCE_COPY_DEFAULT_STRING
  if (message_.IsDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited())) {
    message_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), "", GetArenaForAllocation());
  }
#endif // PROTOBUF_FORCE_COPY_DEFAULT_STRING
  return p;
}
inline void Acknowledge::set_allocated_message(std::string* message) {
  if (message != nullptr) {
    _has_bits_[0] |= 0x00000001u;
  } else {
    _has_bits_[0] &= ~0x00000001u;
  }
  message_.SetAllocated(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), message,
      GetArenaForAllocation());
#ifdef PROTOBUF_FORCE_COPY_DEFAULT_STRING
  if (message_.IsDefault(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited())) {
    message_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), "", GetArenaForAllocation());
  }
#endif // PROTOBUF_FORCE_COPY_DEFAULT_STRING
  // @@protoc_insertion_point(field_set_allocated:indiemotion.messages.Acknowledge.message)
}

#ifdef __GNUC__
  #pragma GCC diagnostic pop
#endif  // __GNUC__

// @@protoc_insertion_point(namespace_scope)

}  // namespace messages
}  // namespace indiemotion

// @@protoc_insertion_point(global_scope)

#include <google/protobuf/port_undef.inc>
#endif  // GOOGLE_PROTOBUF_INCLUDED_GOOGLE_PROTOBUF_INCLUDED_indiemotion_2dproto_2fmessages_2facknowledge_2eproto