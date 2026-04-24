// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'dto.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$CoreStatus {

 String get version;@JsonKey(name: 'uptime_seconds') int get uptimeSeconds;@JsonKey(name: 'tpm_used') bool get tpmUsed;@JsonKey(name: 'db_ok') bool get dbOk;@JsonKey(name: 'last_enforcement_error') String? get lastEnforcementError;
/// Create a copy of CoreStatus
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CoreStatusCopyWith<CoreStatus> get copyWith => _$CoreStatusCopyWithImpl<CoreStatus>(this as CoreStatus, _$identity);

  /// Serializes this CoreStatus to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CoreStatus&&(identical(other.version, version) || other.version == version)&&(identical(other.uptimeSeconds, uptimeSeconds) || other.uptimeSeconds == uptimeSeconds)&&(identical(other.tpmUsed, tpmUsed) || other.tpmUsed == tpmUsed)&&(identical(other.dbOk, dbOk) || other.dbOk == dbOk)&&(identical(other.lastEnforcementError, lastEnforcementError) || other.lastEnforcementError == lastEnforcementError));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,version,uptimeSeconds,tpmUsed,dbOk,lastEnforcementError);

@override
String toString() {
  return 'CoreStatus(version: $version, uptimeSeconds: $uptimeSeconds, tpmUsed: $tpmUsed, dbOk: $dbOk, lastEnforcementError: $lastEnforcementError)';
}


}

/// @nodoc
abstract mixin class $CoreStatusCopyWith<$Res>  {
  factory $CoreStatusCopyWith(CoreStatus value, $Res Function(CoreStatus) _then) = _$CoreStatusCopyWithImpl;
@useResult
$Res call({
 String version,@JsonKey(name: 'uptime_seconds') int uptimeSeconds,@JsonKey(name: 'tpm_used') bool tpmUsed,@JsonKey(name: 'db_ok') bool dbOk,@JsonKey(name: 'last_enforcement_error') String? lastEnforcementError
});




}
/// @nodoc
class _$CoreStatusCopyWithImpl<$Res>
    implements $CoreStatusCopyWith<$Res> {
  _$CoreStatusCopyWithImpl(this._self, this._then);

  final CoreStatus _self;
  final $Res Function(CoreStatus) _then;

/// Create a copy of CoreStatus
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? version = null,Object? uptimeSeconds = null,Object? tpmUsed = null,Object? dbOk = null,Object? lastEnforcementError = freezed,}) {
  return _then(_self.copyWith(
version: null == version ? _self.version : version // ignore: cast_nullable_to_non_nullable
as String,uptimeSeconds: null == uptimeSeconds ? _self.uptimeSeconds : uptimeSeconds // ignore: cast_nullable_to_non_nullable
as int,tpmUsed: null == tpmUsed ? _self.tpmUsed : tpmUsed // ignore: cast_nullable_to_non_nullable
as bool,dbOk: null == dbOk ? _self.dbOk : dbOk // ignore: cast_nullable_to_non_nullable
as bool,lastEnforcementError: freezed == lastEnforcementError ? _self.lastEnforcementError : lastEnforcementError // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}

}


/// Adds pattern-matching-related methods to [CoreStatus].
extension CoreStatusPatterns on CoreStatus {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CoreStatus value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CoreStatus() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CoreStatus value)  $default,){
final _that = this;
switch (_that) {
case _CoreStatus():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CoreStatus value)?  $default,){
final _that = this;
switch (_that) {
case _CoreStatus() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String version, @JsonKey(name: 'uptime_seconds')  int uptimeSeconds, @JsonKey(name: 'tpm_used')  bool tpmUsed, @JsonKey(name: 'db_ok')  bool dbOk, @JsonKey(name: 'last_enforcement_error')  String? lastEnforcementError)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CoreStatus() when $default != null:
return $default(_that.version,_that.uptimeSeconds,_that.tpmUsed,_that.dbOk,_that.lastEnforcementError);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String version, @JsonKey(name: 'uptime_seconds')  int uptimeSeconds, @JsonKey(name: 'tpm_used')  bool tpmUsed, @JsonKey(name: 'db_ok')  bool dbOk, @JsonKey(name: 'last_enforcement_error')  String? lastEnforcementError)  $default,) {final _that = this;
switch (_that) {
case _CoreStatus():
return $default(_that.version,_that.uptimeSeconds,_that.tpmUsed,_that.dbOk,_that.lastEnforcementError);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String version, @JsonKey(name: 'uptime_seconds')  int uptimeSeconds, @JsonKey(name: 'tpm_used')  bool tpmUsed, @JsonKey(name: 'db_ok')  bool dbOk, @JsonKey(name: 'last_enforcement_error')  String? lastEnforcementError)?  $default,) {final _that = this;
switch (_that) {
case _CoreStatus() when $default != null:
return $default(_that.version,_that.uptimeSeconds,_that.tpmUsed,_that.dbOk,_that.lastEnforcementError);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _CoreStatus implements CoreStatus {
  const _CoreStatus({required this.version, @JsonKey(name: 'uptime_seconds') required this.uptimeSeconds, @JsonKey(name: 'tpm_used') required this.tpmUsed, @JsonKey(name: 'db_ok') required this.dbOk, @JsonKey(name: 'last_enforcement_error') this.lastEnforcementError});
  factory _CoreStatus.fromJson(Map<String, dynamic> json) => _$CoreStatusFromJson(json);

@override final  String version;
@override@JsonKey(name: 'uptime_seconds') final  int uptimeSeconds;
@override@JsonKey(name: 'tpm_used') final  bool tpmUsed;
@override@JsonKey(name: 'db_ok') final  bool dbOk;
@override@JsonKey(name: 'last_enforcement_error') final  String? lastEnforcementError;

/// Create a copy of CoreStatus
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CoreStatusCopyWith<_CoreStatus> get copyWith => __$CoreStatusCopyWithImpl<_CoreStatus>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$CoreStatusToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CoreStatus&&(identical(other.version, version) || other.version == version)&&(identical(other.uptimeSeconds, uptimeSeconds) || other.uptimeSeconds == uptimeSeconds)&&(identical(other.tpmUsed, tpmUsed) || other.tpmUsed == tpmUsed)&&(identical(other.dbOk, dbOk) || other.dbOk == dbOk)&&(identical(other.lastEnforcementError, lastEnforcementError) || other.lastEnforcementError == lastEnforcementError));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,version,uptimeSeconds,tpmUsed,dbOk,lastEnforcementError);

@override
String toString() {
  return 'CoreStatus(version: $version, uptimeSeconds: $uptimeSeconds, tpmUsed: $tpmUsed, dbOk: $dbOk, lastEnforcementError: $lastEnforcementError)';
}


}

/// @nodoc
abstract mixin class _$CoreStatusCopyWith<$Res> implements $CoreStatusCopyWith<$Res> {
  factory _$CoreStatusCopyWith(_CoreStatus value, $Res Function(_CoreStatus) _then) = __$CoreStatusCopyWithImpl;
@override @useResult
$Res call({
 String version,@JsonKey(name: 'uptime_seconds') int uptimeSeconds,@JsonKey(name: 'tpm_used') bool tpmUsed,@JsonKey(name: 'db_ok') bool dbOk,@JsonKey(name: 'last_enforcement_error') String? lastEnforcementError
});




}
/// @nodoc
class __$CoreStatusCopyWithImpl<$Res>
    implements _$CoreStatusCopyWith<$Res> {
  __$CoreStatusCopyWithImpl(this._self, this._then);

  final _CoreStatus _self;
  final $Res Function(_CoreStatus) _then;

/// Create a copy of CoreStatus
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? version = null,Object? uptimeSeconds = null,Object? tpmUsed = null,Object? dbOk = null,Object? lastEnforcementError = freezed,}) {
  return _then(_CoreStatus(
version: null == version ? _self.version : version // ignore: cast_nullable_to_non_nullable
as String,uptimeSeconds: null == uptimeSeconds ? _self.uptimeSeconds : uptimeSeconds // ignore: cast_nullable_to_non_nullable
as int,tpmUsed: null == tpmUsed ? _self.tpmUsed : tpmUsed // ignore: cast_nullable_to_non_nullable
as bool,dbOk: null == dbOk ? _self.dbOk : dbOk // ignore: cast_nullable_to_non_nullable
as bool,lastEnforcementError: freezed == lastEnforcementError ? _self.lastEnforcementError : lastEnforcementError // ignore: cast_nullable_to_non_nullable
as String?,
  ));
}


}


/// @nodoc
mixin _$ChildStatus {

@JsonKey(name: 'today_minutes_used') int get todayMinutesUsed;@JsonKey(name: 'today_budget_minutes') int? get todayBudgetMinutes;@JsonKey(name: 'current_window_open') bool get currentWindowOpen;@JsonKey(name: 'current_window_ends_at') DateTime? get currentWindowEndsAt;@JsonKey(name: 'active_blocklist_display') List<String> get activeBlocklistDisplay;@JsonKey(name: 'session_running') bool get sessionRunning;
/// Create a copy of ChildStatus
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ChildStatusCopyWith<ChildStatus> get copyWith => _$ChildStatusCopyWithImpl<ChildStatus>(this as ChildStatus, _$identity);

  /// Serializes this ChildStatus to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ChildStatus&&(identical(other.todayMinutesUsed, todayMinutesUsed) || other.todayMinutesUsed == todayMinutesUsed)&&(identical(other.todayBudgetMinutes, todayBudgetMinutes) || other.todayBudgetMinutes == todayBudgetMinutes)&&(identical(other.currentWindowOpen, currentWindowOpen) || other.currentWindowOpen == currentWindowOpen)&&(identical(other.currentWindowEndsAt, currentWindowEndsAt) || other.currentWindowEndsAt == currentWindowEndsAt)&&const DeepCollectionEquality().equals(other.activeBlocklistDisplay, activeBlocklistDisplay)&&(identical(other.sessionRunning, sessionRunning) || other.sessionRunning == sessionRunning));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,todayMinutesUsed,todayBudgetMinutes,currentWindowOpen,currentWindowEndsAt,const DeepCollectionEquality().hash(activeBlocklistDisplay),sessionRunning);

@override
String toString() {
  return 'ChildStatus(todayMinutesUsed: $todayMinutesUsed, todayBudgetMinutes: $todayBudgetMinutes, currentWindowOpen: $currentWindowOpen, currentWindowEndsAt: $currentWindowEndsAt, activeBlocklistDisplay: $activeBlocklistDisplay, sessionRunning: $sessionRunning)';
}


}

/// @nodoc
abstract mixin class $ChildStatusCopyWith<$Res>  {
  factory $ChildStatusCopyWith(ChildStatus value, $Res Function(ChildStatus) _then) = _$ChildStatusCopyWithImpl;
@useResult
$Res call({
@JsonKey(name: 'today_minutes_used') int todayMinutesUsed,@JsonKey(name: 'today_budget_minutes') int? todayBudgetMinutes,@JsonKey(name: 'current_window_open') bool currentWindowOpen,@JsonKey(name: 'current_window_ends_at') DateTime? currentWindowEndsAt,@JsonKey(name: 'active_blocklist_display') List<String> activeBlocklistDisplay,@JsonKey(name: 'session_running') bool sessionRunning
});




}
/// @nodoc
class _$ChildStatusCopyWithImpl<$Res>
    implements $ChildStatusCopyWith<$Res> {
  _$ChildStatusCopyWithImpl(this._self, this._then);

  final ChildStatus _self;
  final $Res Function(ChildStatus) _then;

/// Create a copy of ChildStatus
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? todayMinutesUsed = null,Object? todayBudgetMinutes = freezed,Object? currentWindowOpen = null,Object? currentWindowEndsAt = freezed,Object? activeBlocklistDisplay = null,Object? sessionRunning = null,}) {
  return _then(_self.copyWith(
todayMinutesUsed: null == todayMinutesUsed ? _self.todayMinutesUsed : todayMinutesUsed // ignore: cast_nullable_to_non_nullable
as int,todayBudgetMinutes: freezed == todayBudgetMinutes ? _self.todayBudgetMinutes : todayBudgetMinutes // ignore: cast_nullable_to_non_nullable
as int?,currentWindowOpen: null == currentWindowOpen ? _self.currentWindowOpen : currentWindowOpen // ignore: cast_nullable_to_non_nullable
as bool,currentWindowEndsAt: freezed == currentWindowEndsAt ? _self.currentWindowEndsAt : currentWindowEndsAt // ignore: cast_nullable_to_non_nullable
as DateTime?,activeBlocklistDisplay: null == activeBlocklistDisplay ? _self.activeBlocklistDisplay : activeBlocklistDisplay // ignore: cast_nullable_to_non_nullable
as List<String>,sessionRunning: null == sessionRunning ? _self.sessionRunning : sessionRunning // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [ChildStatus].
extension ChildStatusPatterns on ChildStatus {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _ChildStatus value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _ChildStatus() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _ChildStatus value)  $default,){
final _that = this;
switch (_that) {
case _ChildStatus():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _ChildStatus value)?  $default,){
final _that = this;
switch (_that) {
case _ChildStatus() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function(@JsonKey(name: 'today_minutes_used')  int todayMinutesUsed, @JsonKey(name: 'today_budget_minutes')  int? todayBudgetMinutes, @JsonKey(name: 'current_window_open')  bool currentWindowOpen, @JsonKey(name: 'current_window_ends_at')  DateTime? currentWindowEndsAt, @JsonKey(name: 'active_blocklist_display')  List<String> activeBlocklistDisplay, @JsonKey(name: 'session_running')  bool sessionRunning)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _ChildStatus() when $default != null:
return $default(_that.todayMinutesUsed,_that.todayBudgetMinutes,_that.currentWindowOpen,_that.currentWindowEndsAt,_that.activeBlocklistDisplay,_that.sessionRunning);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function(@JsonKey(name: 'today_minutes_used')  int todayMinutesUsed, @JsonKey(name: 'today_budget_minutes')  int? todayBudgetMinutes, @JsonKey(name: 'current_window_open')  bool currentWindowOpen, @JsonKey(name: 'current_window_ends_at')  DateTime? currentWindowEndsAt, @JsonKey(name: 'active_blocklist_display')  List<String> activeBlocklistDisplay, @JsonKey(name: 'session_running')  bool sessionRunning)  $default,) {final _that = this;
switch (_that) {
case _ChildStatus():
return $default(_that.todayMinutesUsed,_that.todayBudgetMinutes,_that.currentWindowOpen,_that.currentWindowEndsAt,_that.activeBlocklistDisplay,_that.sessionRunning);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function(@JsonKey(name: 'today_minutes_used')  int todayMinutesUsed, @JsonKey(name: 'today_budget_minutes')  int? todayBudgetMinutes, @JsonKey(name: 'current_window_open')  bool currentWindowOpen, @JsonKey(name: 'current_window_ends_at')  DateTime? currentWindowEndsAt, @JsonKey(name: 'active_blocklist_display')  List<String> activeBlocklistDisplay, @JsonKey(name: 'session_running')  bool sessionRunning)?  $default,) {final _that = this;
switch (_that) {
case _ChildStatus() when $default != null:
return $default(_that.todayMinutesUsed,_that.todayBudgetMinutes,_that.currentWindowOpen,_that.currentWindowEndsAt,_that.activeBlocklistDisplay,_that.sessionRunning);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _ChildStatus implements ChildStatus {
  const _ChildStatus({@JsonKey(name: 'today_minutes_used') required this.todayMinutesUsed, @JsonKey(name: 'today_budget_minutes') this.todayBudgetMinutes, @JsonKey(name: 'current_window_open') required this.currentWindowOpen, @JsonKey(name: 'current_window_ends_at') this.currentWindowEndsAt, @JsonKey(name: 'active_blocklist_display') required final  List<String> activeBlocklistDisplay, @JsonKey(name: 'session_running') required this.sessionRunning}): _activeBlocklistDisplay = activeBlocklistDisplay;
  factory _ChildStatus.fromJson(Map<String, dynamic> json) => _$ChildStatusFromJson(json);

@override@JsonKey(name: 'today_minutes_used') final  int todayMinutesUsed;
@override@JsonKey(name: 'today_budget_minutes') final  int? todayBudgetMinutes;
@override@JsonKey(name: 'current_window_open') final  bool currentWindowOpen;
@override@JsonKey(name: 'current_window_ends_at') final  DateTime? currentWindowEndsAt;
 final  List<String> _activeBlocklistDisplay;
@override@JsonKey(name: 'active_blocklist_display') List<String> get activeBlocklistDisplay {
  if (_activeBlocklistDisplay is EqualUnmodifiableListView) return _activeBlocklistDisplay;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_activeBlocklistDisplay);
}

@override@JsonKey(name: 'session_running') final  bool sessionRunning;

/// Create a copy of ChildStatus
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$ChildStatusCopyWith<_ChildStatus> get copyWith => __$ChildStatusCopyWithImpl<_ChildStatus>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$ChildStatusToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _ChildStatus&&(identical(other.todayMinutesUsed, todayMinutesUsed) || other.todayMinutesUsed == todayMinutesUsed)&&(identical(other.todayBudgetMinutes, todayBudgetMinutes) || other.todayBudgetMinutes == todayBudgetMinutes)&&(identical(other.currentWindowOpen, currentWindowOpen) || other.currentWindowOpen == currentWindowOpen)&&(identical(other.currentWindowEndsAt, currentWindowEndsAt) || other.currentWindowEndsAt == currentWindowEndsAt)&&const DeepCollectionEquality().equals(other._activeBlocklistDisplay, _activeBlocklistDisplay)&&(identical(other.sessionRunning, sessionRunning) || other.sessionRunning == sessionRunning));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,todayMinutesUsed,todayBudgetMinutes,currentWindowOpen,currentWindowEndsAt,const DeepCollectionEquality().hash(_activeBlocklistDisplay),sessionRunning);

@override
String toString() {
  return 'ChildStatus(todayMinutesUsed: $todayMinutesUsed, todayBudgetMinutes: $todayBudgetMinutes, currentWindowOpen: $currentWindowOpen, currentWindowEndsAt: $currentWindowEndsAt, activeBlocklistDisplay: $activeBlocklistDisplay, sessionRunning: $sessionRunning)';
}


}

/// @nodoc
abstract mixin class _$ChildStatusCopyWith<$Res> implements $ChildStatusCopyWith<$Res> {
  factory _$ChildStatusCopyWith(_ChildStatus value, $Res Function(_ChildStatus) _then) = __$ChildStatusCopyWithImpl;
@override @useResult
$Res call({
@JsonKey(name: 'today_minutes_used') int todayMinutesUsed,@JsonKey(name: 'today_budget_minutes') int? todayBudgetMinutes,@JsonKey(name: 'current_window_open') bool currentWindowOpen,@JsonKey(name: 'current_window_ends_at') DateTime? currentWindowEndsAt,@JsonKey(name: 'active_blocklist_display') List<String> activeBlocklistDisplay,@JsonKey(name: 'session_running') bool sessionRunning
});




}
/// @nodoc
class __$ChildStatusCopyWithImpl<$Res>
    implements _$ChildStatusCopyWith<$Res> {
  __$ChildStatusCopyWithImpl(this._self, this._then);

  final _ChildStatus _self;
  final $Res Function(_ChildStatus) _then;

/// Create a copy of ChildStatus
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? todayMinutesUsed = null,Object? todayBudgetMinutes = freezed,Object? currentWindowOpen = null,Object? currentWindowEndsAt = freezed,Object? activeBlocklistDisplay = null,Object? sessionRunning = null,}) {
  return _then(_ChildStatus(
todayMinutesUsed: null == todayMinutesUsed ? _self.todayMinutesUsed : todayMinutesUsed // ignore: cast_nullable_to_non_nullable
as int,todayBudgetMinutes: freezed == todayBudgetMinutes ? _self.todayBudgetMinutes : todayBudgetMinutes // ignore: cast_nullable_to_non_nullable
as int?,currentWindowOpen: null == currentWindowOpen ? _self.currentWindowOpen : currentWindowOpen // ignore: cast_nullable_to_non_nullable
as bool,currentWindowEndsAt: freezed == currentWindowEndsAt ? _self.currentWindowEndsAt : currentWindowEndsAt // ignore: cast_nullable_to_non_nullable
as DateTime?,activeBlocklistDisplay: null == activeBlocklistDisplay ? _self._activeBlocklistDisplay : activeBlocklistDisplay // ignore: cast_nullable_to_non_nullable
as List<String>,sessionRunning: null == sessionRunning ? _self.sessionRunning : sessionRunning // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}


/// @nodoc
mixin _$AppUsage {

@JsonKey(name: 'content_hash') String get contentHash;@JsonKey(name: 'display_name') String? get displayName; String get basename; int get minutes;
/// Create a copy of AppUsage
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AppUsageCopyWith<AppUsage> get copyWith => _$AppUsageCopyWithImpl<AppUsage>(this as AppUsage, _$identity);

  /// Serializes this AppUsage to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppUsage&&(identical(other.contentHash, contentHash) || other.contentHash == contentHash)&&(identical(other.displayName, displayName) || other.displayName == displayName)&&(identical(other.basename, basename) || other.basename == basename)&&(identical(other.minutes, minutes) || other.minutes == minutes));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,contentHash,displayName,basename,minutes);

@override
String toString() {
  return 'AppUsage(contentHash: $contentHash, displayName: $displayName, basename: $basename, minutes: $minutes)';
}


}

/// @nodoc
abstract mixin class $AppUsageCopyWith<$Res>  {
  factory $AppUsageCopyWith(AppUsage value, $Res Function(AppUsage) _then) = _$AppUsageCopyWithImpl;
@useResult
$Res call({
@JsonKey(name: 'content_hash') String contentHash,@JsonKey(name: 'display_name') String? displayName, String basename, int minutes
});




}
/// @nodoc
class _$AppUsageCopyWithImpl<$Res>
    implements $AppUsageCopyWith<$Res> {
  _$AppUsageCopyWithImpl(this._self, this._then);

  final AppUsage _self;
  final $Res Function(AppUsage) _then;

/// Create a copy of AppUsage
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? contentHash = null,Object? displayName = freezed,Object? basename = null,Object? minutes = null,}) {
  return _then(_self.copyWith(
contentHash: null == contentHash ? _self.contentHash : contentHash // ignore: cast_nullable_to_non_nullable
as String,displayName: freezed == displayName ? _self.displayName : displayName // ignore: cast_nullable_to_non_nullable
as String?,basename: null == basename ? _self.basename : basename // ignore: cast_nullable_to_non_nullable
as String,minutes: null == minutes ? _self.minutes : minutes // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [AppUsage].
extension AppUsagePatterns on AppUsage {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _AppUsage value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _AppUsage() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _AppUsage value)  $default,){
final _that = this;
switch (_that) {
case _AppUsage():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _AppUsage value)?  $default,){
final _that = this;
switch (_that) {
case _AppUsage() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function(@JsonKey(name: 'content_hash')  String contentHash, @JsonKey(name: 'display_name')  String? displayName,  String basename,  int minutes)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _AppUsage() when $default != null:
return $default(_that.contentHash,_that.displayName,_that.basename,_that.minutes);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function(@JsonKey(name: 'content_hash')  String contentHash, @JsonKey(name: 'display_name')  String? displayName,  String basename,  int minutes)  $default,) {final _that = this;
switch (_that) {
case _AppUsage():
return $default(_that.contentHash,_that.displayName,_that.basename,_that.minutes);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function(@JsonKey(name: 'content_hash')  String contentHash, @JsonKey(name: 'display_name')  String? displayName,  String basename,  int minutes)?  $default,) {final _that = this;
switch (_that) {
case _AppUsage() when $default != null:
return $default(_that.contentHash,_that.displayName,_that.basename,_that.minutes);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _AppUsage implements AppUsage {
  const _AppUsage({@JsonKey(name: 'content_hash') required this.contentHash, @JsonKey(name: 'display_name') this.displayName, required this.basename, required this.minutes});
  factory _AppUsage.fromJson(Map<String, dynamic> json) => _$AppUsageFromJson(json);

@override@JsonKey(name: 'content_hash') final  String contentHash;
@override@JsonKey(name: 'display_name') final  String? displayName;
@override final  String basename;
@override final  int minutes;

/// Create a copy of AppUsage
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$AppUsageCopyWith<_AppUsage> get copyWith => __$AppUsageCopyWithImpl<_AppUsage>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$AppUsageToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _AppUsage&&(identical(other.contentHash, contentHash) || other.contentHash == contentHash)&&(identical(other.displayName, displayName) || other.displayName == displayName)&&(identical(other.basename, basename) || other.basename == basename)&&(identical(other.minutes, minutes) || other.minutes == minutes));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,contentHash,displayName,basename,minutes);

@override
String toString() {
  return 'AppUsage(contentHash: $contentHash, displayName: $displayName, basename: $basename, minutes: $minutes)';
}


}

/// @nodoc
abstract mixin class _$AppUsageCopyWith<$Res> implements $AppUsageCopyWith<$Res> {
  factory _$AppUsageCopyWith(_AppUsage value, $Res Function(_AppUsage) _then) = __$AppUsageCopyWithImpl;
@override @useResult
$Res call({
@JsonKey(name: 'content_hash') String contentHash,@JsonKey(name: 'display_name') String? displayName, String basename, int minutes
});




}
/// @nodoc
class __$AppUsageCopyWithImpl<$Res>
    implements _$AppUsageCopyWith<$Res> {
  __$AppUsageCopyWithImpl(this._self, this._then);

  final _AppUsage _self;
  final $Res Function(_AppUsage) _then;

/// Create a copy of AppUsage
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? contentHash = null,Object? displayName = freezed,Object? basename = null,Object? minutes = null,}) {
  return _then(_AppUsage(
contentHash: null == contentHash ? _self.contentHash : contentHash // ignore: cast_nullable_to_non_nullable
as String,displayName: freezed == displayName ? _self.displayName : displayName // ignore: cast_nullable_to_non_nullable
as String?,basename: null == basename ? _self.basename : basename // ignore: cast_nullable_to_non_nullable
as String,minutes: null == minutes ? _self.minutes : minutes // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}


/// @nodoc
mixin _$DailyReport {

 DateTime get date;@JsonKey(name: 'usage_by_app') List<AppUsage> get usageByApp;@JsonKey(name: 'total_minutes') int get totalMinutes;
/// Create a copy of DailyReport
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DailyReportCopyWith<DailyReport> get copyWith => _$DailyReportCopyWithImpl<DailyReport>(this as DailyReport, _$identity);

  /// Serializes this DailyReport to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DailyReport&&(identical(other.date, date) || other.date == date)&&const DeepCollectionEquality().equals(other.usageByApp, usageByApp)&&(identical(other.totalMinutes, totalMinutes) || other.totalMinutes == totalMinutes));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,date,const DeepCollectionEquality().hash(usageByApp),totalMinutes);

@override
String toString() {
  return 'DailyReport(date: $date, usageByApp: $usageByApp, totalMinutes: $totalMinutes)';
}


}

/// @nodoc
abstract mixin class $DailyReportCopyWith<$Res>  {
  factory $DailyReportCopyWith(DailyReport value, $Res Function(DailyReport) _then) = _$DailyReportCopyWithImpl;
@useResult
$Res call({
 DateTime date,@JsonKey(name: 'usage_by_app') List<AppUsage> usageByApp,@JsonKey(name: 'total_minutes') int totalMinutes
});




}
/// @nodoc
class _$DailyReportCopyWithImpl<$Res>
    implements $DailyReportCopyWith<$Res> {
  _$DailyReportCopyWithImpl(this._self, this._then);

  final DailyReport _self;
  final $Res Function(DailyReport) _then;

/// Create a copy of DailyReport
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? date = null,Object? usageByApp = null,Object? totalMinutes = null,}) {
  return _then(_self.copyWith(
date: null == date ? _self.date : date // ignore: cast_nullable_to_non_nullable
as DateTime,usageByApp: null == usageByApp ? _self.usageByApp : usageByApp // ignore: cast_nullable_to_non_nullable
as List<AppUsage>,totalMinutes: null == totalMinutes ? _self.totalMinutes : totalMinutes // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [DailyReport].
extension DailyReportPatterns on DailyReport {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _DailyReport value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _DailyReport() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _DailyReport value)  $default,){
final _that = this;
switch (_that) {
case _DailyReport():
return $default(_that);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _DailyReport value)?  $default,){
final _that = this;
switch (_that) {
case _DailyReport() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( DateTime date, @JsonKey(name: 'usage_by_app')  List<AppUsage> usageByApp, @JsonKey(name: 'total_minutes')  int totalMinutes)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _DailyReport() when $default != null:
return $default(_that.date,_that.usageByApp,_that.totalMinutes);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( DateTime date, @JsonKey(name: 'usage_by_app')  List<AppUsage> usageByApp, @JsonKey(name: 'total_minutes')  int totalMinutes)  $default,) {final _that = this;
switch (_that) {
case _DailyReport():
return $default(_that.date,_that.usageByApp,_that.totalMinutes);case _:
  throw StateError('Unexpected subclass');

}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( DateTime date, @JsonKey(name: 'usage_by_app')  List<AppUsage> usageByApp, @JsonKey(name: 'total_minutes')  int totalMinutes)?  $default,) {final _that = this;
switch (_that) {
case _DailyReport() when $default != null:
return $default(_that.date,_that.usageByApp,_that.totalMinutes);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _DailyReport implements DailyReport {
  const _DailyReport({required this.date, @JsonKey(name: 'usage_by_app') required final  List<AppUsage> usageByApp, @JsonKey(name: 'total_minutes') required this.totalMinutes}): _usageByApp = usageByApp;
  factory _DailyReport.fromJson(Map<String, dynamic> json) => _$DailyReportFromJson(json);

@override final  DateTime date;
 final  List<AppUsage> _usageByApp;
@override@JsonKey(name: 'usage_by_app') List<AppUsage> get usageByApp {
  if (_usageByApp is EqualUnmodifiableListView) return _usageByApp;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_usageByApp);
}

@override@JsonKey(name: 'total_minutes') final  int totalMinutes;

/// Create a copy of DailyReport
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$DailyReportCopyWith<_DailyReport> get copyWith => __$DailyReportCopyWithImpl<_DailyReport>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$DailyReportToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _DailyReport&&(identical(other.date, date) || other.date == date)&&const DeepCollectionEquality().equals(other._usageByApp, _usageByApp)&&(identical(other.totalMinutes, totalMinutes) || other.totalMinutes == totalMinutes));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,date,const DeepCollectionEquality().hash(_usageByApp),totalMinutes);

@override
String toString() {
  return 'DailyReport(date: $date, usageByApp: $usageByApp, totalMinutes: $totalMinutes)';
}


}

/// @nodoc
abstract mixin class _$DailyReportCopyWith<$Res> implements $DailyReportCopyWith<$Res> {
  factory _$DailyReportCopyWith(_DailyReport value, $Res Function(_DailyReport) _then) = __$DailyReportCopyWithImpl;
@override @useResult
$Res call({
 DateTime date,@JsonKey(name: 'usage_by_app') List<AppUsage> usageByApp,@JsonKey(name: 'total_minutes') int totalMinutes
});




}
/// @nodoc
class __$DailyReportCopyWithImpl<$Res>
    implements _$DailyReportCopyWith<$Res> {
  __$DailyReportCopyWithImpl(this._self, this._then);

  final _DailyReport _self;
  final $Res Function(_DailyReport) _then;

/// Create a copy of DailyReport
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? date = null,Object? usageByApp = null,Object? totalMinutes = null,}) {
  return _then(_DailyReport(
date: null == date ? _self.date : date // ignore: cast_nullable_to_non_nullable
as DateTime,usageByApp: null == usageByApp ? _self._usageByApp : usageByApp // ignore: cast_nullable_to_non_nullable
as List<AppUsage>,totalMinutes: null == totalMinutes ? _self.totalMinutes : totalMinutes // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

// dart format on
