// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'dto.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

CoreStatus _$CoreStatusFromJson(Map<String, dynamic> json) {
  return _CoreStatus.fromJson(json);
}

/// @nodoc
mixin _$CoreStatus {
  String get version => throw _privateConstructorUsedError;
  @JsonKey(name: 'uptime_seconds')
  int get uptimeSeconds => throw _privateConstructorUsedError;
  @JsonKey(name: 'tpm_used')
  bool get tpmUsed => throw _privateConstructorUsedError;
  @JsonKey(name: 'db_ok')
  bool get dbOk => throw _privateConstructorUsedError;
  @JsonKey(name: 'last_enforcement_error')
  String? get lastEnforcementError => throw _privateConstructorUsedError;

  /// Serializes this CoreStatus to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CoreStatus
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CoreStatusCopyWith<CoreStatus> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CoreStatusCopyWith<$Res> {
  factory $CoreStatusCopyWith(
          CoreStatus value, $Res Function(CoreStatus) then) =
      _$CoreStatusCopyWithImpl<$Res, CoreStatus>;
  @useResult
  $Res call(
      {String version,
      @JsonKey(name: 'uptime_seconds') int uptimeSeconds,
      @JsonKey(name: 'tpm_used') bool tpmUsed,
      @JsonKey(name: 'db_ok') bool dbOk,
      @JsonKey(name: 'last_enforcement_error') String? lastEnforcementError});
}

/// @nodoc
class _$CoreStatusCopyWithImpl<$Res, $Val extends CoreStatus>
    implements $CoreStatusCopyWith<$Res> {
  _$CoreStatusCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CoreStatus
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? version = null,
    Object? uptimeSeconds = null,
    Object? tpmUsed = null,
    Object? dbOk = null,
    Object? lastEnforcementError = freezed,
  }) {
    return _then(_value.copyWith(
      version: null == version
          ? _value.version
          : version // ignore: cast_nullable_to_non_nullable
              as String,
      uptimeSeconds: null == uptimeSeconds
          ? _value.uptimeSeconds
          : uptimeSeconds // ignore: cast_nullable_to_non_nullable
              as int,
      tpmUsed: null == tpmUsed
          ? _value.tpmUsed
          : tpmUsed // ignore: cast_nullable_to_non_nullable
              as bool,
      dbOk: null == dbOk
          ? _value.dbOk
          : dbOk // ignore: cast_nullable_to_non_nullable
              as bool,
      lastEnforcementError: freezed == lastEnforcementError
          ? _value.lastEnforcementError
          : lastEnforcementError // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$CoreStatusImplCopyWith<$Res>
    implements $CoreStatusCopyWith<$Res> {
  factory _$$CoreStatusImplCopyWith(
          _$CoreStatusImpl value, $Res Function(_$CoreStatusImpl) then) =
      __$$CoreStatusImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String version,
      @JsonKey(name: 'uptime_seconds') int uptimeSeconds,
      @JsonKey(name: 'tpm_used') bool tpmUsed,
      @JsonKey(name: 'db_ok') bool dbOk,
      @JsonKey(name: 'last_enforcement_error') String? lastEnforcementError});
}

/// @nodoc
class __$$CoreStatusImplCopyWithImpl<$Res>
    extends _$CoreStatusCopyWithImpl<$Res, _$CoreStatusImpl>
    implements _$$CoreStatusImplCopyWith<$Res> {
  __$$CoreStatusImplCopyWithImpl(
      _$CoreStatusImpl _value, $Res Function(_$CoreStatusImpl) _then)
      : super(_value, _then);

  /// Create a copy of CoreStatus
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? version = null,
    Object? uptimeSeconds = null,
    Object? tpmUsed = null,
    Object? dbOk = null,
    Object? lastEnforcementError = freezed,
  }) {
    return _then(_$CoreStatusImpl(
      version: null == version
          ? _value.version
          : version // ignore: cast_nullable_to_non_nullable
              as String,
      uptimeSeconds: null == uptimeSeconds
          ? _value.uptimeSeconds
          : uptimeSeconds // ignore: cast_nullable_to_non_nullable
              as int,
      tpmUsed: null == tpmUsed
          ? _value.tpmUsed
          : tpmUsed // ignore: cast_nullable_to_non_nullable
              as bool,
      dbOk: null == dbOk
          ? _value.dbOk
          : dbOk // ignore: cast_nullable_to_non_nullable
              as bool,
      lastEnforcementError: freezed == lastEnforcementError
          ? _value.lastEnforcementError
          : lastEnforcementError // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CoreStatusImpl implements _CoreStatus {
  const _$CoreStatusImpl(
      {required this.version,
      @JsonKey(name: 'uptime_seconds') required this.uptimeSeconds,
      @JsonKey(name: 'tpm_used') required this.tpmUsed,
      @JsonKey(name: 'db_ok') required this.dbOk,
      @JsonKey(name: 'last_enforcement_error') this.lastEnforcementError});

  factory _$CoreStatusImpl.fromJson(Map<String, dynamic> json) =>
      _$$CoreStatusImplFromJson(json);

  @override
  final String version;
  @override
  @JsonKey(name: 'uptime_seconds')
  final int uptimeSeconds;
  @override
  @JsonKey(name: 'tpm_used')
  final bool tpmUsed;
  @override
  @JsonKey(name: 'db_ok')
  final bool dbOk;
  @override
  @JsonKey(name: 'last_enforcement_error')
  final String? lastEnforcementError;

  @override
  String toString() {
    return 'CoreStatus(version: $version, uptimeSeconds: $uptimeSeconds, tpmUsed: $tpmUsed, dbOk: $dbOk, lastEnforcementError: $lastEnforcementError)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CoreStatusImpl &&
            (identical(other.version, version) || other.version == version) &&
            (identical(other.uptimeSeconds, uptimeSeconds) ||
                other.uptimeSeconds == uptimeSeconds) &&
            (identical(other.tpmUsed, tpmUsed) || other.tpmUsed == tpmUsed) &&
            (identical(other.dbOk, dbOk) || other.dbOk == dbOk) &&
            (identical(other.lastEnforcementError, lastEnforcementError) ||
                other.lastEnforcementError == lastEnforcementError));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType, version, uptimeSeconds, tpmUsed, dbOk, lastEnforcementError);

  /// Create a copy of CoreStatus
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CoreStatusImplCopyWith<_$CoreStatusImpl> get copyWith =>
      __$$CoreStatusImplCopyWithImpl<_$CoreStatusImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CoreStatusImplToJson(
      this,
    );
  }
}

abstract class _CoreStatus implements CoreStatus {
  const factory _CoreStatus(
      {required final String version,
      @JsonKey(name: 'uptime_seconds') required final int uptimeSeconds,
      @JsonKey(name: 'tpm_used') required final bool tpmUsed,
      @JsonKey(name: 'db_ok') required final bool dbOk,
      @JsonKey(name: 'last_enforcement_error')
      final String? lastEnforcementError}) = _$CoreStatusImpl;

  factory _CoreStatus.fromJson(Map<String, dynamic> json) =
      _$CoreStatusImpl.fromJson;

  @override
  String get version;
  @override
  @JsonKey(name: 'uptime_seconds')
  int get uptimeSeconds;
  @override
  @JsonKey(name: 'tpm_used')
  bool get tpmUsed;
  @override
  @JsonKey(name: 'db_ok')
  bool get dbOk;
  @override
  @JsonKey(name: 'last_enforcement_error')
  String? get lastEnforcementError;

  /// Create a copy of CoreStatus
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CoreStatusImplCopyWith<_$CoreStatusImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

ChildStatus _$ChildStatusFromJson(Map<String, dynamic> json) {
  return _ChildStatus.fromJson(json);
}

/// @nodoc
mixin _$ChildStatus {
  @JsonKey(name: 'today_minutes_used')
  int get todayMinutesUsed => throw _privateConstructorUsedError;
  @JsonKey(name: 'today_budget_minutes')
  int? get todayBudgetMinutes => throw _privateConstructorUsedError;
  @JsonKey(name: 'current_window_open')
  bool get currentWindowOpen => throw _privateConstructorUsedError;
  @JsonKey(name: 'current_window_ends_at')
  DateTime? get currentWindowEndsAt => throw _privateConstructorUsedError;
  @JsonKey(name: 'active_blocklist_display')
  List<String> get activeBlocklistDisplay => throw _privateConstructorUsedError;
  @JsonKey(name: 'session_running')
  bool get sessionRunning => throw _privateConstructorUsedError;

  /// Serializes this ChildStatus to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of ChildStatus
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ChildStatusCopyWith<ChildStatus> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ChildStatusCopyWith<$Res> {
  factory $ChildStatusCopyWith(
          ChildStatus value, $Res Function(ChildStatus) then) =
      _$ChildStatusCopyWithImpl<$Res, ChildStatus>;
  @useResult
  $Res call(
      {@JsonKey(name: 'today_minutes_used') int todayMinutesUsed,
      @JsonKey(name: 'today_budget_minutes') int? todayBudgetMinutes,
      @JsonKey(name: 'current_window_open') bool currentWindowOpen,
      @JsonKey(name: 'current_window_ends_at') DateTime? currentWindowEndsAt,
      @JsonKey(name: 'active_blocklist_display')
      List<String> activeBlocklistDisplay,
      @JsonKey(name: 'session_running') bool sessionRunning});
}

/// @nodoc
class _$ChildStatusCopyWithImpl<$Res, $Val extends ChildStatus>
    implements $ChildStatusCopyWith<$Res> {
  _$ChildStatusCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ChildStatus
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? todayMinutesUsed = null,
    Object? todayBudgetMinutes = freezed,
    Object? currentWindowOpen = null,
    Object? currentWindowEndsAt = freezed,
    Object? activeBlocklistDisplay = null,
    Object? sessionRunning = null,
  }) {
    return _then(_value.copyWith(
      todayMinutesUsed: null == todayMinutesUsed
          ? _value.todayMinutesUsed
          : todayMinutesUsed // ignore: cast_nullable_to_non_nullable
              as int,
      todayBudgetMinutes: freezed == todayBudgetMinutes
          ? _value.todayBudgetMinutes
          : todayBudgetMinutes // ignore: cast_nullable_to_non_nullable
              as int?,
      currentWindowOpen: null == currentWindowOpen
          ? _value.currentWindowOpen
          : currentWindowOpen // ignore: cast_nullable_to_non_nullable
              as bool,
      currentWindowEndsAt: freezed == currentWindowEndsAt
          ? _value.currentWindowEndsAt
          : currentWindowEndsAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      activeBlocklistDisplay: null == activeBlocklistDisplay
          ? _value.activeBlocklistDisplay
          : activeBlocklistDisplay // ignore: cast_nullable_to_non_nullable
              as List<String>,
      sessionRunning: null == sessionRunning
          ? _value.sessionRunning
          : sessionRunning // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$ChildStatusImplCopyWith<$Res>
    implements $ChildStatusCopyWith<$Res> {
  factory _$$ChildStatusImplCopyWith(
          _$ChildStatusImpl value, $Res Function(_$ChildStatusImpl) then) =
      __$$ChildStatusImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {@JsonKey(name: 'today_minutes_used') int todayMinutesUsed,
      @JsonKey(name: 'today_budget_minutes') int? todayBudgetMinutes,
      @JsonKey(name: 'current_window_open') bool currentWindowOpen,
      @JsonKey(name: 'current_window_ends_at') DateTime? currentWindowEndsAt,
      @JsonKey(name: 'active_blocklist_display')
      List<String> activeBlocklistDisplay,
      @JsonKey(name: 'session_running') bool sessionRunning});
}

/// @nodoc
class __$$ChildStatusImplCopyWithImpl<$Res>
    extends _$ChildStatusCopyWithImpl<$Res, _$ChildStatusImpl>
    implements _$$ChildStatusImplCopyWith<$Res> {
  __$$ChildStatusImplCopyWithImpl(
      _$ChildStatusImpl _value, $Res Function(_$ChildStatusImpl) _then)
      : super(_value, _then);

  /// Create a copy of ChildStatus
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? todayMinutesUsed = null,
    Object? todayBudgetMinutes = freezed,
    Object? currentWindowOpen = null,
    Object? currentWindowEndsAt = freezed,
    Object? activeBlocklistDisplay = null,
    Object? sessionRunning = null,
  }) {
    return _then(_$ChildStatusImpl(
      todayMinutesUsed: null == todayMinutesUsed
          ? _value.todayMinutesUsed
          : todayMinutesUsed // ignore: cast_nullable_to_non_nullable
              as int,
      todayBudgetMinutes: freezed == todayBudgetMinutes
          ? _value.todayBudgetMinutes
          : todayBudgetMinutes // ignore: cast_nullable_to_non_nullable
              as int?,
      currentWindowOpen: null == currentWindowOpen
          ? _value.currentWindowOpen
          : currentWindowOpen // ignore: cast_nullable_to_non_nullable
              as bool,
      currentWindowEndsAt: freezed == currentWindowEndsAt
          ? _value.currentWindowEndsAt
          : currentWindowEndsAt // ignore: cast_nullable_to_non_nullable
              as DateTime?,
      activeBlocklistDisplay: null == activeBlocklistDisplay
          ? _value._activeBlocklistDisplay
          : activeBlocklistDisplay // ignore: cast_nullable_to_non_nullable
              as List<String>,
      sessionRunning: null == sessionRunning
          ? _value.sessionRunning
          : sessionRunning // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$ChildStatusImpl implements _ChildStatus {
  const _$ChildStatusImpl(
      {@JsonKey(name: 'today_minutes_used') required this.todayMinutesUsed,
      @JsonKey(name: 'today_budget_minutes') this.todayBudgetMinutes,
      @JsonKey(name: 'current_window_open') required this.currentWindowOpen,
      @JsonKey(name: 'current_window_ends_at') this.currentWindowEndsAt,
      @JsonKey(name: 'active_blocklist_display')
      required final List<String> activeBlocklistDisplay,
      @JsonKey(name: 'session_running') required this.sessionRunning})
      : _activeBlocklistDisplay = activeBlocklistDisplay;

  factory _$ChildStatusImpl.fromJson(Map<String, dynamic> json) =>
      _$$ChildStatusImplFromJson(json);

  @override
  @JsonKey(name: 'today_minutes_used')
  final int todayMinutesUsed;
  @override
  @JsonKey(name: 'today_budget_minutes')
  final int? todayBudgetMinutes;
  @override
  @JsonKey(name: 'current_window_open')
  final bool currentWindowOpen;
  @override
  @JsonKey(name: 'current_window_ends_at')
  final DateTime? currentWindowEndsAt;
  final List<String> _activeBlocklistDisplay;
  @override
  @JsonKey(name: 'active_blocklist_display')
  List<String> get activeBlocklistDisplay {
    if (_activeBlocklistDisplay is EqualUnmodifiableListView)
      return _activeBlocklistDisplay;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_activeBlocklistDisplay);
  }

  @override
  @JsonKey(name: 'session_running')
  final bool sessionRunning;

  @override
  String toString() {
    return 'ChildStatus(todayMinutesUsed: $todayMinutesUsed, todayBudgetMinutes: $todayBudgetMinutes, currentWindowOpen: $currentWindowOpen, currentWindowEndsAt: $currentWindowEndsAt, activeBlocklistDisplay: $activeBlocklistDisplay, sessionRunning: $sessionRunning)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ChildStatusImpl &&
            (identical(other.todayMinutesUsed, todayMinutesUsed) ||
                other.todayMinutesUsed == todayMinutesUsed) &&
            (identical(other.todayBudgetMinutes, todayBudgetMinutes) ||
                other.todayBudgetMinutes == todayBudgetMinutes) &&
            (identical(other.currentWindowOpen, currentWindowOpen) ||
                other.currentWindowOpen == currentWindowOpen) &&
            (identical(other.currentWindowEndsAt, currentWindowEndsAt) ||
                other.currentWindowEndsAt == currentWindowEndsAt) &&
            const DeepCollectionEquality().equals(
                other._activeBlocklistDisplay, _activeBlocklistDisplay) &&
            (identical(other.sessionRunning, sessionRunning) ||
                other.sessionRunning == sessionRunning));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(
      runtimeType,
      todayMinutesUsed,
      todayBudgetMinutes,
      currentWindowOpen,
      currentWindowEndsAt,
      const DeepCollectionEquality().hash(_activeBlocklistDisplay),
      sessionRunning);

  /// Create a copy of ChildStatus
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ChildStatusImplCopyWith<_$ChildStatusImpl> get copyWith =>
      __$$ChildStatusImplCopyWithImpl<_$ChildStatusImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$ChildStatusImplToJson(
      this,
    );
  }
}

abstract class _ChildStatus implements ChildStatus {
  const factory _ChildStatus(
      {@JsonKey(name: 'today_minutes_used') required final int todayMinutesUsed,
      @JsonKey(name: 'today_budget_minutes') final int? todayBudgetMinutes,
      @JsonKey(name: 'current_window_open')
      required final bool currentWindowOpen,
      @JsonKey(name: 'current_window_ends_at')
      final DateTime? currentWindowEndsAt,
      @JsonKey(name: 'active_blocklist_display')
      required final List<String> activeBlocklistDisplay,
      @JsonKey(name: 'session_running')
      required final bool sessionRunning}) = _$ChildStatusImpl;

  factory _ChildStatus.fromJson(Map<String, dynamic> json) =
      _$ChildStatusImpl.fromJson;

  @override
  @JsonKey(name: 'today_minutes_used')
  int get todayMinutesUsed;
  @override
  @JsonKey(name: 'today_budget_minutes')
  int? get todayBudgetMinutes;
  @override
  @JsonKey(name: 'current_window_open')
  bool get currentWindowOpen;
  @override
  @JsonKey(name: 'current_window_ends_at')
  DateTime? get currentWindowEndsAt;
  @override
  @JsonKey(name: 'active_blocklist_display')
  List<String> get activeBlocklistDisplay;
  @override
  @JsonKey(name: 'session_running')
  bool get sessionRunning;

  /// Create a copy of ChildStatus
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ChildStatusImplCopyWith<_$ChildStatusImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

AppUsage _$AppUsageFromJson(Map<String, dynamic> json) {
  return _AppUsage.fromJson(json);
}

/// @nodoc
mixin _$AppUsage {
  @JsonKey(name: 'content_hash')
  String get contentHash => throw _privateConstructorUsedError;
  @JsonKey(name: 'display_name')
  String? get displayName => throw _privateConstructorUsedError;
  String get basename => throw _privateConstructorUsedError;
  int get minutes => throw _privateConstructorUsedError;

  /// Serializes this AppUsage to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of AppUsage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $AppUsageCopyWith<AppUsage> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AppUsageCopyWith<$Res> {
  factory $AppUsageCopyWith(AppUsage value, $Res Function(AppUsage) then) =
      _$AppUsageCopyWithImpl<$Res, AppUsage>;
  @useResult
  $Res call(
      {@JsonKey(name: 'content_hash') String contentHash,
      @JsonKey(name: 'display_name') String? displayName,
      String basename,
      int minutes});
}

/// @nodoc
class _$AppUsageCopyWithImpl<$Res, $Val extends AppUsage>
    implements $AppUsageCopyWith<$Res> {
  _$AppUsageCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of AppUsage
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? contentHash = null,
    Object? displayName = freezed,
    Object? basename = null,
    Object? minutes = null,
  }) {
    return _then(_value.copyWith(
      contentHash: null == contentHash
          ? _value.contentHash
          : contentHash // ignore: cast_nullable_to_non_nullable
              as String,
      displayName: freezed == displayName
          ? _value.displayName
          : displayName // ignore: cast_nullable_to_non_nullable
              as String?,
      basename: null == basename
          ? _value.basename
          : basename // ignore: cast_nullable_to_non_nullable
              as String,
      minutes: null == minutes
          ? _value.minutes
          : minutes // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$AppUsageImplCopyWith<$Res>
    implements $AppUsageCopyWith<$Res> {
  factory _$$AppUsageImplCopyWith(
          _$AppUsageImpl value, $Res Function(_$AppUsageImpl) then) =
      __$$AppUsageImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {@JsonKey(name: 'content_hash') String contentHash,
      @JsonKey(name: 'display_name') String? displayName,
      String basename,
      int minutes});
}

/// @nodoc
class __$$AppUsageImplCopyWithImpl<$Res>
    extends _$AppUsageCopyWithImpl<$Res, _$AppUsageImpl>
    implements _$$AppUsageImplCopyWith<$Res> {
  __$$AppUsageImplCopyWithImpl(
      _$AppUsageImpl _value, $Res Function(_$AppUsageImpl) _then)
      : super(_value, _then);

  /// Create a copy of AppUsage
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? contentHash = null,
    Object? displayName = freezed,
    Object? basename = null,
    Object? minutes = null,
  }) {
    return _then(_$AppUsageImpl(
      contentHash: null == contentHash
          ? _value.contentHash
          : contentHash // ignore: cast_nullable_to_non_nullable
              as String,
      displayName: freezed == displayName
          ? _value.displayName
          : displayName // ignore: cast_nullable_to_non_nullable
              as String?,
      basename: null == basename
          ? _value.basename
          : basename // ignore: cast_nullable_to_non_nullable
              as String,
      minutes: null == minutes
          ? _value.minutes
          : minutes // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$AppUsageImpl implements _AppUsage {
  const _$AppUsageImpl(
      {@JsonKey(name: 'content_hash') required this.contentHash,
      @JsonKey(name: 'display_name') this.displayName,
      required this.basename,
      required this.minutes});

  factory _$AppUsageImpl.fromJson(Map<String, dynamic> json) =>
      _$$AppUsageImplFromJson(json);

  @override
  @JsonKey(name: 'content_hash')
  final String contentHash;
  @override
  @JsonKey(name: 'display_name')
  final String? displayName;
  @override
  final String basename;
  @override
  final int minutes;

  @override
  String toString() {
    return 'AppUsage(contentHash: $contentHash, displayName: $displayName, basename: $basename, minutes: $minutes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppUsageImpl &&
            (identical(other.contentHash, contentHash) ||
                other.contentHash == contentHash) &&
            (identical(other.displayName, displayName) ||
                other.displayName == displayName) &&
            (identical(other.basename, basename) ||
                other.basename == basename) &&
            (identical(other.minutes, minutes) || other.minutes == minutes));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, contentHash, displayName, basename, minutes);

  /// Create a copy of AppUsage
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AppUsageImplCopyWith<_$AppUsageImpl> get copyWith =>
      __$$AppUsageImplCopyWithImpl<_$AppUsageImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$AppUsageImplToJson(
      this,
    );
  }
}

abstract class _AppUsage implements AppUsage {
  const factory _AppUsage(
      {@JsonKey(name: 'content_hash') required final String contentHash,
      @JsonKey(name: 'display_name') final String? displayName,
      required final String basename,
      required final int minutes}) = _$AppUsageImpl;

  factory _AppUsage.fromJson(Map<String, dynamic> json) =
      _$AppUsageImpl.fromJson;

  @override
  @JsonKey(name: 'content_hash')
  String get contentHash;
  @override
  @JsonKey(name: 'display_name')
  String? get displayName;
  @override
  String get basename;
  @override
  int get minutes;

  /// Create a copy of AppUsage
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AppUsageImplCopyWith<_$AppUsageImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

DailyReport _$DailyReportFromJson(Map<String, dynamic> json) {
  return _DailyReport.fromJson(json);
}

/// @nodoc
mixin _$DailyReport {
  DateTime get date => throw _privateConstructorUsedError;
  @JsonKey(name: 'usage_by_app')
  List<AppUsage> get usageByApp => throw _privateConstructorUsedError;
  @JsonKey(name: 'total_minutes')
  int get totalMinutes => throw _privateConstructorUsedError;

  /// Serializes this DailyReport to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of DailyReport
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $DailyReportCopyWith<DailyReport> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $DailyReportCopyWith<$Res> {
  factory $DailyReportCopyWith(
          DailyReport value, $Res Function(DailyReport) then) =
      _$DailyReportCopyWithImpl<$Res, DailyReport>;
  @useResult
  $Res call(
      {DateTime date,
      @JsonKey(name: 'usage_by_app') List<AppUsage> usageByApp,
      @JsonKey(name: 'total_minutes') int totalMinutes});
}

/// @nodoc
class _$DailyReportCopyWithImpl<$Res, $Val extends DailyReport>
    implements $DailyReportCopyWith<$Res> {
  _$DailyReportCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of DailyReport
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? date = null,
    Object? usageByApp = null,
    Object? totalMinutes = null,
  }) {
    return _then(_value.copyWith(
      date: null == date
          ? _value.date
          : date // ignore: cast_nullable_to_non_nullable
              as DateTime,
      usageByApp: null == usageByApp
          ? _value.usageByApp
          : usageByApp // ignore: cast_nullable_to_non_nullable
              as List<AppUsage>,
      totalMinutes: null == totalMinutes
          ? _value.totalMinutes
          : totalMinutes // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$DailyReportImplCopyWith<$Res>
    implements $DailyReportCopyWith<$Res> {
  factory _$$DailyReportImplCopyWith(
          _$DailyReportImpl value, $Res Function(_$DailyReportImpl) then) =
      __$$DailyReportImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {DateTime date,
      @JsonKey(name: 'usage_by_app') List<AppUsage> usageByApp,
      @JsonKey(name: 'total_minutes') int totalMinutes});
}

/// @nodoc
class __$$DailyReportImplCopyWithImpl<$Res>
    extends _$DailyReportCopyWithImpl<$Res, _$DailyReportImpl>
    implements _$$DailyReportImplCopyWith<$Res> {
  __$$DailyReportImplCopyWithImpl(
      _$DailyReportImpl _value, $Res Function(_$DailyReportImpl) _then)
      : super(_value, _then);

  /// Create a copy of DailyReport
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? date = null,
    Object? usageByApp = null,
    Object? totalMinutes = null,
  }) {
    return _then(_$DailyReportImpl(
      date: null == date
          ? _value.date
          : date // ignore: cast_nullable_to_non_nullable
              as DateTime,
      usageByApp: null == usageByApp
          ? _value._usageByApp
          : usageByApp // ignore: cast_nullable_to_non_nullable
              as List<AppUsage>,
      totalMinutes: null == totalMinutes
          ? _value.totalMinutes
          : totalMinutes // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$DailyReportImpl implements _DailyReport {
  const _$DailyReportImpl(
      {required this.date,
      @JsonKey(name: 'usage_by_app') required final List<AppUsage> usageByApp,
      @JsonKey(name: 'total_minutes') required this.totalMinutes})
      : _usageByApp = usageByApp;

  factory _$DailyReportImpl.fromJson(Map<String, dynamic> json) =>
      _$$DailyReportImplFromJson(json);

  @override
  final DateTime date;
  final List<AppUsage> _usageByApp;
  @override
  @JsonKey(name: 'usage_by_app')
  List<AppUsage> get usageByApp {
    if (_usageByApp is EqualUnmodifiableListView) return _usageByApp;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_usageByApp);
  }

  @override
  @JsonKey(name: 'total_minutes')
  final int totalMinutes;

  @override
  String toString() {
    return 'DailyReport(date: $date, usageByApp: $usageByApp, totalMinutes: $totalMinutes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$DailyReportImpl &&
            (identical(other.date, date) || other.date == date) &&
            const DeepCollectionEquality()
                .equals(other._usageByApp, _usageByApp) &&
            (identical(other.totalMinutes, totalMinutes) ||
                other.totalMinutes == totalMinutes));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, date,
      const DeepCollectionEquality().hash(_usageByApp), totalMinutes);

  /// Create a copy of DailyReport
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$DailyReportImplCopyWith<_$DailyReportImpl> get copyWith =>
      __$$DailyReportImplCopyWithImpl<_$DailyReportImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$DailyReportImplToJson(
      this,
    );
  }
}

abstract class _DailyReport implements DailyReport {
  const factory _DailyReport(
      {required final DateTime date,
      @JsonKey(name: 'usage_by_app') required final List<AppUsage> usageByApp,
      @JsonKey(name: 'total_minutes')
      required final int totalMinutes}) = _$DailyReportImpl;

  factory _DailyReport.fromJson(Map<String, dynamic> json) =
      _$DailyReportImpl.fromJson;

  @override
  DateTime get date;
  @override
  @JsonKey(name: 'usage_by_app')
  List<AppUsage> get usageByApp;
  @override
  @JsonKey(name: 'total_minutes')
  int get totalMinutes;

  /// Create a copy of DailyReport
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$DailyReportImplCopyWith<_$DailyReportImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
