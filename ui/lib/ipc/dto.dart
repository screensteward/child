import 'package:freezed_annotation/freezed_annotation.dart';

part 'dto.freezed.dart';
part 'dto.g.dart';

@freezed
abstract class CoreStatus with _$CoreStatus {
  const factory CoreStatus({
    required String version,
    @JsonKey(name: 'uptime_seconds') required int uptimeSeconds,
    @JsonKey(name: 'tpm_used') required bool tpmUsed,
    @JsonKey(name: 'db_ok') required bool dbOk,
    @JsonKey(name: 'last_enforcement_error') String? lastEnforcementError,
  }) = _CoreStatus;
  factory CoreStatus.fromJson(Map<String, dynamic> j) =>
      _$CoreStatusFromJson(j);
}

@freezed
abstract class ChildStatus with _$ChildStatus {
  const factory ChildStatus({
    @JsonKey(name: 'today_minutes_used') required int todayMinutesUsed,
    @JsonKey(name: 'today_budget_minutes') int? todayBudgetMinutes,
    @JsonKey(name: 'current_window_open') required bool currentWindowOpen,
    @JsonKey(name: 'current_window_ends_at') DateTime? currentWindowEndsAt,
    @JsonKey(name: 'active_blocklist_display')
    required List<String> activeBlocklistDisplay,
    @JsonKey(name: 'session_running') required bool sessionRunning,
  }) = _ChildStatus;
  factory ChildStatus.fromJson(Map<String, dynamic> j) =>
      _$ChildStatusFromJson(j);
}

@freezed
abstract class AppUsage with _$AppUsage {
  const factory AppUsage({
    @JsonKey(name: 'content_hash') required String contentHash,
    @JsonKey(name: 'display_name') String? displayName,
    required String basename,
    required int minutes,
  }) = _AppUsage;
  factory AppUsage.fromJson(Map<String, dynamic> j) => _$AppUsageFromJson(j);
}

@freezed
abstract class DailyReport with _$DailyReport {
  const factory DailyReport({
    required DateTime date,
    @JsonKey(name: 'usage_by_app') required List<AppUsage> usageByApp,
    @JsonKey(name: 'total_minutes') required int totalMinutes,
  }) = _DailyReport;
  factory DailyReport.fromJson(Map<String, dynamic> j) =>
      _$DailyReportFromJson(j);
}
