//! SportsCraft Internationalization (i18n) Engine.
//!
//! Follows the fleet standard: typed locale enums, zero-panic string lookups,
//! multi-language translations (EN, DE, FR, ES, JA, KO, ZH),
//! and sports biomechanics / kinematics terminology localization.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppLocale {
    #[default]
    En, // English (Global Default)
    De, // German (Austrian / German Fleet native)
    Fr, // French
    Es, // Spanish
    Ja, // Japanese
    Ko, // Korean
    Zh, // Chinese
}

impl AppLocale {
    pub fn all() -> &'static [AppLocale] {
        &[
            AppLocale::En,
            AppLocale::De,
            AppLocale::Fr,
            AppLocale::Es,
            AppLocale::Ja,
            AppLocale::Ko,
            AppLocale::Zh,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppLocale::En => "en",
            AppLocale::De => "de",
            AppLocale::Fr => "fr",
            AppLocale::Es => "es",
            AppLocale::Ja => "ja",
            AppLocale::Ko => "ko",
            AppLocale::Zh => "zh",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppLocale::En => "English (US/UK)",
            AppLocale::De => "Deutsch (DE/AT/CH)",
            AppLocale::Fr => "Français (FR)",
            AppLocale::Es => "Español (ES/LATAM)",
            AppLocale::Ja => "日本語 (JA)",
            AppLocale::Ko => "한국어 (KO)",
            AppLocale::Zh => "中文 (ZH)",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            AppLocale::En => "🇬🇧",
            AppLocale::De => "🇩🇪",
            AppLocale::Fr => "🇫🇷",
            AppLocale::Es => "🇪🇸",
            AppLocale::Ja => "🇯🇵",
            AppLocale::Ko => "🇰🇷",
            AppLocale::Zh => "🇨🇳",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum I18nKey {
    // Navigation & Workspace
    NavKinematics,
    NavTelemetry,
    NavVideoAnalysis,
    NavPerformanceLedger,

    // Metrics & Sensors
    Velocity,
    Acceleration,
    JointAngles,
    PowerOutput,
    HeartRate,
    Cadence,

    // Actions
    ActionStartSession,
    ActionStopSession,
    ActionExportTelemetry,
    StatusReady,
}

pub fn t(key: I18nKey, locale: AppLocale) -> &'static str {
    match (key, locale) {
        (I18nKey::NavKinematics, AppLocale::En) => "🏃 Kinematics & Biomechanics",
        (I18nKey::NavKinematics, AppLocale::De) => "🏃 Kinematik & Biomechanik",
        (I18nKey::NavKinematics, AppLocale::Fr) => "🏃 Cinématique & Biomécanique",
        (I18nKey::NavKinematics, AppLocale::Es) => "🏃 Cinemática y Biomecánica",
        (I18nKey::NavKinematics, AppLocale::Ja) => "🏃 運動学・生体力学解析",
        (I18nKey::NavKinematics, AppLocale::Ko) => "🏃 운동학 및 생체역학",
        (I18nKey::NavKinematics, AppLocale::Zh) => "🏃 运动学与生物力学",

        (I18nKey::NavTelemetry, AppLocale::En) => "📡 Live Telemetry",
        (I18nKey::NavTelemetry, AppLocale::De) => "📡 Live-Telemetrie",
        (I18nKey::NavTelemetry, AppLocale::Fr) => "📡 Télémétrie en direct",
        (I18nKey::NavTelemetry, AppLocale::Es) => "📡 Telemetría en directo",
        (I18nKey::NavTelemetry, AppLocale::Ja) => "📡 リアルタイムテレメトリ",
        (I18nKey::NavTelemetry, AppLocale::Ko) => "📡 실시간 원격측정",
        (I18nKey::NavTelemetry, AppLocale::Zh) => "📡 实时遥测数据",

        (I18nKey::NavVideoAnalysis, AppLocale::En) => "🎥 Motion Pose Tracking",
        (I18nKey::NavVideoAnalysis, AppLocale::De) => "🎥 Bewegungs-Pose-Tracking",
        (I18nKey::NavVideoAnalysis, AppLocale::Fr) => "🎥 Suivi de pose en mouvement",
        (I18nKey::NavVideoAnalysis, AppLocale::Es) => "🎥 Seguimiento de pose de movimiento",
        (I18nKey::NavVideoAnalysis, AppLocale::Ja) => "🎥 骨格姿勢推定・動画解析",
        (I18nKey::NavVideoAnalysis, AppLocale::Ko) => "🎥 동작 포즈 추적",
        (I18nKey::NavVideoAnalysis, AppLocale::Zh) => "🎥 动作姿态追踪与分析",

        (I18nKey::NavPerformanceLedger, AppLocale::En) => "📊 Performance Ledger",
        (I18nKey::NavPerformanceLedger, AppLocale::De) => "📊 Leistungsbuch",
        (I18nKey::NavPerformanceLedger, AppLocale::Fr) => "📊 Registre des performances",
        (I18nKey::NavPerformanceLedger, AppLocale::Es) => "📊 Registro de rendimiento",
        (I18nKey::NavPerformanceLedger, AppLocale::Ja) => "📊 パフォーマンス台帳",
        (I18nKey::NavPerformanceLedger, AppLocale::Ko) => "📊 성과 및 기록 장부",
        (I18nKey::NavPerformanceLedger, AppLocale::Zh) => "📊 运动表现账本",

        (I18nKey::Velocity, AppLocale::En) => "Velocity (m/s)",
        (I18nKey::Velocity, AppLocale::De) => "Geschwindigkeit (m/s)",
        (I18nKey::Velocity, AppLocale::Fr) => "Vitesse (m/s)",
        (I18nKey::Velocity, AppLocale::Es) => "Velocidad (m/s)",
        (I18nKey::Velocity, AppLocale::Ja) => "速度 (m/s)",
        (I18nKey::Velocity, AppLocale::Ko) => "속도 (m/s)",
        (I18nKey::Velocity, AppLocale::Zh) => "速度 (m/s)",

        (I18nKey::Acceleration, AppLocale::En) => "Acceleration (m/s²)",
        (I18nKey::Acceleration, AppLocale::De) => "Beschleunigung (m/s²)",
        (I18nKey::Acceleration, AppLocale::Fr) => "Accélération (m/s²)",
        (I18nKey::Acceleration, AppLocale::Es) => "Aceleración (m/s²)",
        (I18nKey::Acceleration, AppLocale::Ja) => "加速度 (m/s²)",
        (I18nKey::Acceleration, AppLocale::Ko) => "가속도 (m/s²)",
        (I18nKey::Acceleration, AppLocale::Zh) => "加速度 (m/s²)",

        (I18nKey::JointAngles, AppLocale::En) => "Joint Angles (°)",
        (I18nKey::JointAngles, AppLocale::De) => "Gelenkwinkel (°)",
        (I18nKey::JointAngles, AppLocale::Fr) => "Angles articulaires (°)",
        (I18nKey::JointAngles, AppLocale::Es) => "Ángulos articulares (°)",
        (I18nKey::JointAngles, AppLocale::Ja) => "関節角度 (°)",
        (I18nKey::JointAngles, AppLocale::Ko) => "관절 각도 (°)",
        (I18nKey::JointAngles, AppLocale::Zh) => "关节角度 (°)",

        (I18nKey::PowerOutput, AppLocale::En) => "Power Output (W)",
        (I18nKey::PowerOutput, AppLocale::De) => "Leistungsabgabe (W)",
        (I18nKey::PowerOutput, AppLocale::Fr) => "Puissance de sortie (W)",
        (I18nKey::PowerOutput, AppLocale::Es) => "Potencia de salida (W)",
        (I18nKey::PowerOutput, AppLocale::Ja) => "出力パワー (W)",
        (I18nKey::PowerOutput, AppLocale::Ko) => "파워 출력 (W)",
        (I18nKey::PowerOutput, AppLocale::Zh) => "输出功率 (W)",

        (I18nKey::HeartRate, AppLocale::En) => "Heart Rate (BPM)",
        (I18nKey::HeartRate, AppLocale::De) => "Herzfrequenz (BPM)",
        (I18nKey::HeartRate, AppLocale::Fr) => "Fréquence cardiaque (BPM)",
        (I18nKey::HeartRate, AppLocale::Es) => "Frecuencia cardíaca (BPM)",
        (I18nKey::HeartRate, AppLocale::Ja) => "心拍数 (BPM)",
        (I18nKey::HeartRate, AppLocale::Ko) => "심박수 (BPM)",
        (I18nKey::HeartRate, AppLocale::Zh) => "心率 (BPM)",

        (I18nKey::Cadence, AppLocale::En) => "Cadence (RPM)",
        (I18nKey::Cadence, AppLocale::De) => "Trittfrequenz (RPM)",
        (I18nKey::Cadence, AppLocale::Fr) => "Cadence (RPM)",
        (I18nKey::Cadence, AppLocale::Es) => "Cadencia (RPM)",
        (I18nKey::Cadence, AppLocale::Ja) => "ケイデンス (RPM)",
        (I18nKey::Cadence, AppLocale::Ko) => "케이던스 (RPM)",
        (I18nKey::Cadence, AppLocale::Zh) => "踏频 / 步频 (RPM)",

        (I18nKey::ActionStartSession, AppLocale::En) => "Start Telemetry Session",
        (I18nKey::ActionStartSession, AppLocale::De) => "Telemetriesitzung starten",
        (I18nKey::ActionStartSession, AppLocale::Fr) => "Démarrer la session",
        (I18nKey::ActionStartSession, AppLocale::Es) => "Iniciar sesión de telemetría",
        (I18nKey::ActionStartSession, AppLocale::Ja) => "計測セッション開始",
        (I18nKey::ActionStartSession, AppLocale::Ko) => "측정 세션 시작",
        (I18nKey::ActionStartSession, AppLocale::Zh) => "启动遥测会话",

        (I18nKey::ActionStopSession, AppLocale::En) => "Stop & Save Session",
        (I18nKey::ActionStopSession, AppLocale::De) => "Sitzung stoppen & speichern",
        (I18nKey::ActionStopSession, AppLocale::Fr) => "Arrêter et sauvegarder",
        (I18nKey::ActionStopSession, AppLocale::Es) => "Detener y guardar sesión",
        (I18nKey::ActionStopSession, AppLocale::Ja) => "計測停止・データ保存",
        (I18nKey::ActionStopSession, AppLocale::Ko) => "세션 중지 및 저장",
        (I18nKey::ActionStopSession, AppLocale::Zh) => "停止并保存会话",

        (I18nKey::ActionExportTelemetry, AppLocale::En) => "Export Biomechanics Manifest",
        (I18nKey::ActionExportTelemetry, AppLocale::De) => "Biomechanik-Manifest exportieren",
        (I18nKey::ActionExportTelemetry, AppLocale::Fr) => "Exporter le manifeste",
        (I18nKey::ActionExportTelemetry, AppLocale::Es) => "Exportar manifiesto de biomecánica",
        (I18nKey::ActionExportTelemetry, AppLocale::Ja) => "生体力学マニフェスト出力",
        (I18nKey::ActionExportTelemetry, AppLocale::Ko) => "생체역학 데이터 내보내기",
        (I18nKey::ActionExportTelemetry, AppLocale::Zh) => "导出生物力学清单",

        (I18nKey::StatusReady, AppLocale::En) => "SportsCraft Kinematics Engine Ready",
        (I18nKey::StatusReady, AppLocale::De) => "SportsCraft Kinematik-Engine bereit",
        (I18nKey::StatusReady, AppLocale::Fr) => "Moteur cinématique SportsCraft prêt",
        (I18nKey::StatusReady, AppLocale::Es) => "Motor cinemático SportsCraft listo",
        (I18nKey::StatusReady, AppLocale::Ja) => "SportsCraft 運動解析エンジン準備完了",
        (I18nKey::StatusReady, AppLocale::Ko) => "SportsCraft 운동역학 엔진 준비완료",
        (I18nKey::StatusReady, AppLocale::Zh) => "SportsCraft 运动动力学引擎就绪",
    }
}
