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

    // Desktop UI (used by crates/sportscraft-app)
    MenuFile,
    MenuDistances,
    MenuView,
    MenuHelp,
    ActionExportPlan,
    ActionCopyPlan,
    ActionExit,
    ActionAbout,
    ActionThemeLight,
    ActionThemeDark,
    AppTitle,
    PanelBanister,
    PanelGait,
    PanelPacing,
    LabelFitness,
    LabelFatigue,
    LabelForm,
    LabelCadence,
    LabelVerticalOsc,
    LabelGroundContact,
    LabelLegStiffness,
    LabelAsymmetry,
    FieldTargetDistance,
    LabelPredictionResults,
    LabelEstimatedFinish,
    LabelRecommendedPace,
    LabelTsbWindow,
}

pub fn t(key: I18nKey, locale: AppLocale) -> &'static str {
    match (key, locale) {
        // Desktop UI strings.
        (I18nKey::MenuFile, AppLocale::En) => "File",
        (I18nKey::MenuFile, AppLocale::De) => "Datei",
        (I18nKey::MenuFile, AppLocale::Fr) => "Fichier",
        (I18nKey::MenuFile, AppLocale::Es) => "Archivo",
        (I18nKey::MenuFile, AppLocale::Ja) => "ファイル",
        (I18nKey::MenuFile, AppLocale::Ko) => "파일",
        (I18nKey::MenuFile, AppLocale::Zh) => "文件",

        (I18nKey::MenuDistances, AppLocale::En) => "Distances",
        (I18nKey::MenuDistances, AppLocale::De) => "Distanzen",
        (I18nKey::MenuDistances, AppLocale::Fr) => "Distances",
        (I18nKey::MenuDistances, AppLocale::Es) => "Distancias",
        (I18nKey::MenuDistances, AppLocale::Ja) => "距離",
        (I18nKey::MenuDistances, AppLocale::Ko) => "거리",
        (I18nKey::MenuDistances, AppLocale::Zh) => "距离",

        (I18nKey::MenuView, AppLocale::En) => "View",
        (I18nKey::MenuView, AppLocale::De) => "Ansicht",
        (I18nKey::MenuView, AppLocale::Fr) => "Affichage",
        (I18nKey::MenuView, AppLocale::Es) => "Vista",
        (I18nKey::MenuView, AppLocale::Ja) => "表示",
        (I18nKey::MenuView, AppLocale::Ko) => "보기",
        (I18nKey::MenuView, AppLocale::Zh) => "视图",

        (I18nKey::MenuHelp, AppLocale::En) => "Help",
        (I18nKey::MenuHelp, AppLocale::De) => "Hilfe",
        (I18nKey::MenuHelp, AppLocale::Fr) => "Aide",
        (I18nKey::MenuHelp, AppLocale::Es) => "Ayuda",
        (I18nKey::MenuHelp, AppLocale::Ja) => "ヘルプ",
        (I18nKey::MenuHelp, AppLocale::Ko) => "도움말",
        (I18nKey::MenuHelp, AppLocale::Zh) => "帮助",

        (I18nKey::ActionExportPlan, AppLocale::En) => "Export Pacing Plan (JSON)",
        (I18nKey::ActionExportPlan, AppLocale::De) => "Trainingsplan exportieren (JSON)",
        (I18nKey::ActionExportPlan, AppLocale::Fr) => "Exporter le plan (JSON)",
        (I18nKey::ActionExportPlan, AppLocale::Es) => "Exportar plan (JSON)",
        (I18nKey::ActionExportPlan, AppLocale::Ja) => "ペース計画を書き出す (JSON)",
        (I18nKey::ActionExportPlan, AppLocale::Ko) => "페이스 계획 내보내기 (JSON)",
        (I18nKey::ActionExportPlan, AppLocale::Zh) => "导出配速计划 (JSON)",

        (I18nKey::ActionCopyPlan, AppLocale::En) => "Copy Pacing Plan",
        (I18nKey::ActionCopyPlan, AppLocale::De) => "Trainingsplan kopieren",
        (I18nKey::ActionCopyPlan, AppLocale::Fr) => "Copier le plan",
        (I18nKey::ActionCopyPlan, AppLocale::Es) => "Copiar plan",
        (I18nKey::ActionCopyPlan, AppLocale::Ja) => "ペース計画をコピー",
        (I18nKey::ActionCopyPlan, AppLocale::Ko) => "페이스 계획 복사",
        (I18nKey::ActionCopyPlan, AppLocale::Zh) => "复制配速计划",

        (I18nKey::ActionExit, AppLocale::En) => "Exit",
        (I18nKey::ActionExit, AppLocale::De) => "Beenden",
        (I18nKey::ActionExit, AppLocale::Fr) => "Quitter",
        (I18nKey::ActionExit, AppLocale::Es) => "Salir",
        (I18nKey::ActionExit, AppLocale::Ja) => "終了",
        (I18nKey::ActionExit, AppLocale::Ko) => "종료",
        (I18nKey::ActionExit, AppLocale::Zh) => "退出",

        (I18nKey::ActionAbout, AppLocale::En) => "About SportsCraft",
        (I18nKey::ActionAbout, AppLocale::De) => "Über SportsCraft",
        (I18nKey::ActionAbout, AppLocale::Fr) => "À propos de SportsCraft",
        (I18nKey::ActionAbout, AppLocale::Es) => "Acerca de SportsCraft",
        (I18nKey::ActionAbout, AppLocale::Ja) => "SportsCraft について",
        (I18nKey::ActionAbout, AppLocale::Ko) => "SportsCraft 정보",
        (I18nKey::ActionAbout, AppLocale::Zh) => "关于 SportsCraft",

        (I18nKey::ActionThemeLight, AppLocale::En) => "Light",
        (I18nKey::ActionThemeLight, AppLocale::De) => "Hell",
        (I18nKey::ActionThemeLight, AppLocale::Fr) => "Clair",
        (I18nKey::ActionThemeLight, AppLocale::Es) => "Claro",
        (I18nKey::ActionThemeLight, AppLocale::Ja) => "ライト",
        (I18nKey::ActionThemeLight, AppLocale::Ko) => "라이트",
        (I18nKey::ActionThemeLight, AppLocale::Zh) => "浅色",

        (I18nKey::ActionThemeDark, AppLocale::En) => "Dark",
        (I18nKey::ActionThemeDark, AppLocale::De) => "Dunkel",
        (I18nKey::ActionThemeDark, AppLocale::Fr) => "Sombre",
        (I18nKey::ActionThemeDark, AppLocale::Es) => "Oscuro",
        (I18nKey::ActionThemeDark, AppLocale::Ja) => "ダーク",
        (I18nKey::ActionThemeDark, AppLocale::Ko) => "다크",
        (I18nKey::ActionThemeDark, AppLocale::Zh) => "深色",

        (I18nKey::AppTitle, AppLocale::En) => {
            "SportsCraft - Biomechanics, Banister Impulse & Kinematics"
        }
        (I18nKey::AppTitle, AppLocale::De) => {
            "SportsCraft - Biomechanik, Banister-Impuls & Kinematik"
        }
        (I18nKey::AppTitle, AppLocale::Fr) => {
            "SportsCraft - Biomécanique, impulsion de Banister & cinématique"
        }
        (I18nKey::AppTitle, AppLocale::Es) => {
            "SportsCraft - Biomecánica, impulso de Banister y cinemática"
        }
        (I18nKey::AppTitle, AppLocale::Ja) => {
            "SportsCraft - バイオメカニクス・バニスター・キネマティクス"
        }
        (I18nKey::AppTitle, AppLocale::Ko) => {
            "SportsCraft - 생체역학, 배니스터 임펄스 및 키네마틱스"
        }
        (I18nKey::AppTitle, AppLocale::Zh) => "SportsCraft - 生物力学、巴尼斯特冲量与运动学",

        (I18nKey::PanelBanister, AppLocale::En) => "Banister Training Load",
        (I18nKey::PanelBanister, AppLocale::De) => "Banister-Trainingslast",
        (I18nKey::PanelBanister, AppLocale::Fr) => "Charge d'entraînement Banister",
        (I18nKey::PanelBanister, AppLocale::Es) => "Carga de entrenamiento Banister",
        (I18nKey::PanelBanister, AppLocale::Ja) => "バニスター訓練負荷",
        (I18nKey::PanelBanister, AppLocale::Ko) => "배니스터 훈련 부하",
        (I18nKey::PanelBanister, AppLocale::Zh) => "巴尼斯特训练负荷",

        (I18nKey::PanelGait, AppLocale::En) => "IMU Gait Kinematics",
        (I18nKey::PanelGait, AppLocale::De) => "IMU-Gangkinematik",
        (I18nKey::PanelGait, AppLocale::Fr) => "Cinématique de course IMU",
        (I18nKey::PanelGait, AppLocale::Es) => "Cinemática de marcha IMU",
        (I18nKey::PanelGait, AppLocale::Ja) => "IMU 歩行キネマティクス",
        (I18nKey::PanelGait, AppLocale::Ko) => "IMU 보행 키네마틱스",
        (I18nKey::PanelGait, AppLocale::Zh) => "IMU 步态运动学",

        (I18nKey::PanelPacing, AppLocale::En) => "Race Pacing & Critical Velocity",
        (I18nKey::PanelPacing, AppLocale::De) => "Wettkampf-Pacing & kritische Geschwindigkeit",
        (I18nKey::PanelPacing, AppLocale::Fr) => "Allure de course & vitesse critique",
        (I18nKey::PanelPacing, AppLocale::Es) => "Ritmo de carrera y velocidad crítica",
        (I18nKey::PanelPacing, AppLocale::Ja) => "レースペースと臨界速度",
        (I18nKey::PanelPacing, AppLocale::Ko) => "레이스 페이싱 및 임계 속도",
        (I18nKey::PanelPacing, AppLocale::Zh) => "比赛配速与临界速度",

        (I18nKey::LabelFitness, AppLocale::En) => "Fitness (CTL, 42d)",
        (I18nKey::LabelFitness, AppLocale::De) => "Fitness (CTL, 42 T)",
        (I18nKey::LabelFitness, AppLocale::Fr) => "Forme (CTL, 42 j)",
        (I18nKey::LabelFitness, AppLocale::Es) => "Forma física (CTL, 42 d)",
        (I18nKey::LabelFitness, AppLocale::Ja) => "フィットネス (CTL, 42日)",
        (I18nKey::LabelFitness, AppLocale::Ko) => "피트니스 (CTL, 42일)",
        (I18nKey::LabelFitness, AppLocale::Zh) => "体能 (CTL, 42天)",

        (I18nKey::LabelFatigue, AppLocale::En) => "Fatigue (ATL, 7d)",
        (I18nKey::LabelFatigue, AppLocale::De) => "Ermüdung (ATL, 7 T)",
        (I18nKey::LabelFatigue, AppLocale::Fr) => "Fatigue (ATL, 7 j)",
        (I18nKey::LabelFatigue, AppLocale::Es) => "Fatiga (ATL, 7 d)",
        (I18nKey::LabelFatigue, AppLocale::Ja) => "疲労 (ATL, 7日)",
        (I18nKey::LabelFatigue, AppLocale::Ko) => "피로 (ATL, 7일)",
        (I18nKey::LabelFatigue, AppLocale::Zh) => "疲劳 (ATL, 7天)",

        (I18nKey::LabelForm, AppLocale::En) => "Form (TSB)",
        (I18nKey::LabelForm, AppLocale::De) => "Form (TSB)",
        (I18nKey::LabelForm, AppLocale::Fr) => "Condition (TSB)",
        (I18nKey::LabelForm, AppLocale::Es) => "Estado de forma (TSB)",
        (I18nKey::LabelForm, AppLocale::Ja) => "コンディション (TSB)",
        (I18nKey::LabelForm, AppLocale::Ko) => "컨디션 (TSB)",
        (I18nKey::LabelForm, AppLocale::Zh) => "状态 (TSB)",

        (I18nKey::LabelCadence, AppLocale::En) => "Cadence",
        (I18nKey::LabelCadence, AppLocale::De) => "Kadenz",
        (I18nKey::LabelCadence, AppLocale::Fr) => "Cadence",
        (I18nKey::LabelCadence, AppLocale::Es) => "Cadencia",
        (I18nKey::LabelCadence, AppLocale::Ja) => "ケイデンス",
        (I18nKey::LabelCadence, AppLocale::Ko) => "케이던스",
        (I18nKey::LabelCadence, AppLocale::Zh) => "步频",

        (I18nKey::LabelVerticalOsc, AppLocale::En) => "Vertical Oscillation",
        (I18nKey::LabelVerticalOsc, AppLocale::De) => "Vertikale Schwingung",
        (I18nKey::LabelVerticalOsc, AppLocale::Fr) => "Oscillation verticale",
        (I18nKey::LabelVerticalOsc, AppLocale::Es) => "Oscilación vertical",
        (I18nKey::LabelVerticalOsc, AppLocale::Ja) => "上下動",
        (I18nKey::LabelVerticalOsc, AppLocale::Ko) => "수직 진동",
        (I18nKey::LabelVerticalOsc, AppLocale::Zh) => "垂直振幅",

        (I18nKey::LabelGroundContact, AppLocale::En) => "Ground Contact Time",
        (I18nKey::LabelGroundContact, AppLocale::De) => "Bodenkontaktzeit",
        (I18nKey::LabelGroundContact, AppLocale::Fr) => "Temps de contact au sol",
        (I18nKey::LabelGroundContact, AppLocale::Es) => "Tiempo de contacto",
        (I18nKey::LabelGroundContact, AppLocale::Ja) => "接地時間",
        (I18nKey::LabelGroundContact, AppLocale::Ko) => "지면 접촉 시간",
        (I18nKey::LabelGroundContact, AppLocale::Zh) => "触地时间",

        (I18nKey::LabelLegStiffness, AppLocale::En) => "Leg Spring Stiffness",
        (I18nKey::LabelLegStiffness, AppLocale::De) => "Beinfedersteifigkeit",
        (I18nKey::LabelLegStiffness, AppLocale::Fr) => "Raideur du ressort de jambe",
        (I18nKey::LabelLegStiffness, AppLocale::Es) => "Rigidez del resorte de pierna",
        (I18nKey::LabelLegStiffness, AppLocale::Ja) => "脚バネ剛性",
        (I18nKey::LabelLegStiffness, AppLocale::Ko) => "다리 스프링 강성",
        (I18nKey::LabelLegStiffness, AppLocale::Zh) => "腿部弹簧刚度",

        (I18nKey::LabelAsymmetry, AppLocale::En) => "Asymmetry",
        (I18nKey::LabelAsymmetry, AppLocale::De) => "Asymmetrie",
        (I18nKey::LabelAsymmetry, AppLocale::Fr) => "Asymétrie",
        (I18nKey::LabelAsymmetry, AppLocale::Es) => "Asimetría",
        (I18nKey::LabelAsymmetry, AppLocale::Ja) => "非対称性",
        (I18nKey::LabelAsymmetry, AppLocale::Ko) => "비대칭",
        (I18nKey::LabelAsymmetry, AppLocale::Zh) => "不对称性",

        (I18nKey::FieldTargetDistance, AppLocale::En) => "Target Distance (meters)",
        (I18nKey::FieldTargetDistance, AppLocale::De) => "Zieldistanz (Meter)",
        (I18nKey::FieldTargetDistance, AppLocale::Fr) => "Distance cible (mètres)",
        (I18nKey::FieldTargetDistance, AppLocale::Es) => "Distancia objetivo (metros)",
        (I18nKey::FieldTargetDistance, AppLocale::Ja) => "目標距離 (メートル)",
        (I18nKey::FieldTargetDistance, AppLocale::Ko) => "목표 거리 (미터)",
        (I18nKey::FieldTargetDistance, AppLocale::Zh) => "目标距离 (米)",

        (I18nKey::LabelPredictionResults, AppLocale::En) => "Prediction Results",
        (I18nKey::LabelPredictionResults, AppLocale::De) => "Prognoseergebnisse",
        (I18nKey::LabelPredictionResults, AppLocale::Fr) => "Résultats de prédiction",
        (I18nKey::LabelPredictionResults, AppLocale::Es) => "Resultados de predicción",
        (I18nKey::LabelPredictionResults, AppLocale::Ja) => "予測結果",
        (I18nKey::LabelPredictionResults, AppLocale::Ko) => "예측 결과",
        (I18nKey::LabelPredictionResults, AppLocale::Zh) => "预测结果",

        (I18nKey::LabelEstimatedFinish, AppLocale::En) => "Estimated Finish",
        (I18nKey::LabelEstimatedFinish, AppLocale::De) => "Geschätzte Zielzeit",
        (I18nKey::LabelEstimatedFinish, AppLocale::Fr) => "Temps estimé",
        (I18nKey::LabelEstimatedFinish, AppLocale::Es) => "Tiempo estimado",
        (I18nKey::LabelEstimatedFinish, AppLocale::Ja) => "予測フィニッシュタイム",
        (I18nKey::LabelEstimatedFinish, AppLocale::Ko) => "예상 완주 시간",
        (I18nKey::LabelEstimatedFinish, AppLocale::Zh) => "预计完赛时间",

        (I18nKey::LabelRecommendedPace, AppLocale::En) => "Recommended Pace",
        (I18nKey::LabelRecommendedPace, AppLocale::De) => "Empfohlenes Tempo",
        (I18nKey::LabelRecommendedPace, AppLocale::Fr) => "Allure recommandée",
        (I18nKey::LabelRecommendedPace, AppLocale::Es) => "Ritmo recomendado",
        (I18nKey::LabelRecommendedPace, AppLocale::Ja) => "推奨ペース",
        (I18nKey::LabelRecommendedPace, AppLocale::Ko) => "권장 페이스",
        (I18nKey::LabelRecommendedPace, AppLocale::Zh) => "建议配速",

        (I18nKey::LabelTsbWindow, AppLocale::En) => "Target Form TSB Window",
        (I18nKey::LabelTsbWindow, AppLocale::De) => "Ziel-TSB-Fenster",
        (I18nKey::LabelTsbWindow, AppLocale::Fr) => "Fenêtre TSB cible",
        (I18nKey::LabelTsbWindow, AppLocale::Es) => "Ventana TSB objetivo",
        (I18nKey::LabelTsbWindow, AppLocale::Ja) => "目標TSBウィンドウ",
        (I18nKey::LabelTsbWindow, AppLocale::Ko) => "목표 TSB 구간",
        (I18nKey::LabelTsbWindow, AppLocale::Zh) => "目标 TSB 区间",

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
