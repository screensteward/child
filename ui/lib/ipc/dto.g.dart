// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'dto.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$CoreStatusImpl _$$CoreStatusImplFromJson(Map<String, dynamic> json) =>
    _$CoreStatusImpl(
      version: json['version'] as String,
      uptimeSeconds: (json['uptime_seconds'] as num).toInt(),
      tpmUsed: json['tpm_used'] as bool,
      dbOk: json['db_ok'] as bool,
      lastEnforcementError: json['last_enforcement_error'] as String?,
    );

Map<String, dynamic> _$$CoreStatusImplToJson(_$CoreStatusImpl instance) =>
    <String, dynamic>{
      'version': instance.version,
      'uptime_seconds': instance.uptimeSeconds,
      'tpm_used': instance.tpmUsed,
      'db_ok': instance.dbOk,
      'last_enforcement_error': instance.lastEnforcementError,
    };

_$ChildStatusImpl _$$ChildStatusImplFromJson(Map<String, dynamic> json) =>
    _$ChildStatusImpl(
      todayMinutesUsed: (json['today_minutes_used'] as num).toInt(),
      todayBudgetMinutes: (json['today_budget_minutes'] as num?)?.toInt(),
      currentWindowOpen: json['current_window_open'] as bool,
      currentWindowEndsAt: json['current_window_ends_at'] == null
          ? null
          : DateTime.parse(json['current_window_ends_at'] as String),
      activeBlocklistDisplay:
          (json['active_blocklist_display'] as List<dynamic>)
              .map((e) => e as String)
              .toList(),
      sessionRunning: json['session_running'] as bool,
    );

Map<String, dynamic> _$$ChildStatusImplToJson(_$ChildStatusImpl instance) =>
    <String, dynamic>{
      'today_minutes_used': instance.todayMinutesUsed,
      'today_budget_minutes': instance.todayBudgetMinutes,
      'current_window_open': instance.currentWindowOpen,
      'current_window_ends_at': instance.currentWindowEndsAt?.toIso8601String(),
      'active_blocklist_display': instance.activeBlocklistDisplay,
      'session_running': instance.sessionRunning,
    };

_$AppUsageImpl _$$AppUsageImplFromJson(Map<String, dynamic> json) =>
    _$AppUsageImpl(
      contentHash: json['content_hash'] as String,
      displayName: json['display_name'] as String?,
      basename: json['basename'] as String,
      minutes: (json['minutes'] as num).toInt(),
    );

Map<String, dynamic> _$$AppUsageImplToJson(_$AppUsageImpl instance) =>
    <String, dynamic>{
      'content_hash': instance.contentHash,
      'display_name': instance.displayName,
      'basename': instance.basename,
      'minutes': instance.minutes,
    };

_$DailyReportImpl _$$DailyReportImplFromJson(Map<String, dynamic> json) =>
    _$DailyReportImpl(
      date: DateTime.parse(json['date'] as String),
      usageByApp: (json['usage_by_app'] as List<dynamic>)
          .map((e) => AppUsage.fromJson(e as Map<String, dynamic>))
          .toList(),
      totalMinutes: (json['total_minutes'] as num).toInt(),
    );

Map<String, dynamic> _$$DailyReportImplToJson(_$DailyReportImpl instance) =>
    <String, dynamic>{
      'date': instance.date.toIso8601String(),
      'usage_by_app': instance.usageByApp,
      'total_minutes': instance.totalMinutes,
    };
