//! Trait-based UI translations.

use crate::models::Locale;

pub trait Strings {
    fn app_title(&self) -> &'static str;
    fn ready(&self) -> &'static str;
    fn no_selection(&self) -> &'static str;
    fn context_label(&self, path: &str) -> String;
    fn selected_tag_context(&self) -> &'static str;
    fn selected_repository_context(&self) -> &'static str;
    fn selected_registry_context(&self) -> &'static str;
    fn context_guidance(&self) -> &'static str;
    fn refresh(&self) -> &'static str;
    fn settings(&self) -> &'static str;
    fn close_settings(&self) -> &'static str;
    fn open_settings(&self) -> &'static str;

    fn settings_title(&self) -> &'static str;
    fn settings_subtitle(&self) -> &'static str;
    fn appearance(&self) -> &'static str;
    fn theme(&self) -> &'static str;
    fn language(&self) -> &'static str;
    fn follow_system(&self) -> &'static str;
    fn light(&self) -> &'static str;
    fn dark(&self) -> &'static str;
    fn english(&self) -> &'static str;
    fn chinese_simplified(&self) -> &'static str;
    fn chinese_traditional(&self) -> &'static str;
    fn cache(&self) -> &'static str;
    fn auto_refresh_interval(&self) -> &'static str;
    fn cache_max_age(&self) -> &'static str;
    fn reset_cache_settings(&self) -> &'static str;
    fn import_export(&self) -> &'static str;
    fn export_registries(&self) -> &'static str;
    fn generate_export(&self) -> &'static str;
    fn import_registries(&self) -> &'static str;
    fn paste_exported_json_here(&self) -> &'static str;
    fn import_action(&self) -> &'static str;

    fn registries(&self) -> &'static str;
    fn registries_subtitle(&self) -> &'static str;
    fn add_registry_button(&self) -> &'static str;
    fn filter_by_name_or_url(&self) -> &'static str;
    fn no_registries_configured(&self) -> &'static str;
    fn no_registries_match_filter(&self) -> &'static str;
    fn delete_registry(&self) -> &'static str;
    fn delete_registry_confirm(&self, registry_name: &str) -> String;
    fn delete_registry_warning(&self) -> &'static str;
    fn connected(&self) -> &'static str;
    fn disconnected(&self) -> &'static str;
    fn error(&self) -> &'static str;
    fn not_checked(&self) -> &'static str;
    fn edit_registry(&self) -> &'static str;
    fn add_registry(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn url(&self) -> &'static str;
    fn authentication(&self) -> &'static str;
    fn anonymous(&self) -> &'static str;
    fn basic_auth(&self) -> &'static str;
    fn bearer_token(&self) -> &'static str;
    fn username(&self) -> &'static str;
    fn password(&self) -> &'static str;
    fn token(&self) -> &'static str;
    fn cancel(&self) -> &'static str;
    fn save(&self) -> &'static str;
    fn registry_url_placeholder(&self) -> &'static str;

    fn repositories(&self) -> &'static str;
    fn repositories_subtitle(&self) -> &'static str;
    fn select_registry_to_view_repositories(&self) -> &'static str;
    fn search_repositories(&self) -> &'static str;
    fn loading(&self) -> &'static str;
    fn retry(&self) -> &'static str;
    fn no_repositories_found(&self) -> &'static str;
    fn no_matching_repositories(&self) -> &'static str;
    fn delete_repository_title(&self) -> &'static str;
    fn failed_to_fetch_repositories(&self, err: &str) -> String;
    fn failed_to_create_client(&self, err: &str) -> String;
    fn failed_to_fetch_tags(&self, err: &str) -> String;
    fn client_error(&self, err: &str) -> String;
    fn deleted_tags_successfully(&self, deleted: usize) -> String;
    fn deleted_with_failures(&self, deleted: usize, failed: usize) -> String;

    fn tags(&self) -> &'static str;
    fn tags_subtitle(&self) -> &'static str;
    fn select_repository_to_view_tags(&self) -> &'static str;
    fn search_tags(&self) -> &'static str;
    fn selected_count(&self, count: usize) -> String;
    fn delete_selected(&self) -> &'static str;
    fn deleting(&self) -> &'static str;
    fn deleted_tags(&self, deleted: usize) -> String;
    fn deleted_tags_with_errors(&self, deleted: usize, errors: usize) -> String;
    fn error_with_details(&self, err: &str) -> String;
    fn no_tags_found(&self) -> &'static str;
    fn no_matching_tags(&self) -> &'static str;
    fn delete_tags(&self) -> &'static str;
    fn delete_tags_confirm(&self, count: usize) -> String;
    fn more_items(&self, count: usize) -> String;
    fn action_cannot_be_undone(&self) -> &'static str;
    fn delete_action(&self) -> &'static str;

    fn manifest_details(&self) -> &'static str;
    fn select_tag_to_inspect(&self) -> &'static str;
    fn select_tag_guidance(&self) -> &'static str;
    fn select_tag_body(&self) -> &'static str;
    fn loading_manifest_summary(&self, tag: &str) -> String;
    fn failed_manifest_summary(&self, tag: &str) -> String;
    fn pending_manifest_summary(&self, tag: &str) -> String;
    fn detail_summary(&self, tag: &str, layers: usize) -> String;
    fn loading_manifest(&self) -> &'static str;
    fn overview(&self) -> &'static str;
    fn tag(&self) -> &'static str;
    fn digest(&self) -> &'static str;
    fn media_type(&self) -> &'static str;
    fn total_size(&self) -> &'static str;
    fn layers(&self, count: usize) -> String;
    fn raw_json(&self) -> &'static str;
    fn raw_json_help(&self) -> &'static str;
    fn show_raw_json(&self) -> &'static str;
    fn close(&self) -> &'static str;
    fn failed_to_fetch_manifest(&self, err: &str) -> String;
    fn build_history(&self) -> &'static str;
    fn no_history_available(&self) -> &'static str;
    fn empty_layer(&self) -> &'static str;

    fn repository_deletion_completed(&self) -> &'static str;
    fn deleted_of_tags(&self, deleted: usize, total: usize) -> String;
    fn failed_to_delete_tags(&self, failed: usize) -> String;
    fn deleting_tags(&self) -> &'static str;
    fn delete_repository_confirm(&self, repo_name: &str) -> String;
    fn delete_all_tags_warning(&self, total_tags: usize) -> String;
    fn delete_all_tags(&self) -> &'static str;
    fn no_digest_returned(&self) -> &'static str;
    fn repository_has_no_tags(&self, repo_name: &str) -> String;
    fn empty_repository_hint(&self) -> &'static str;
    fn api_repository_delete_not_supported(&self) -> &'static str;
    fn garbage_collect_hint(&self) -> &'static str;
    fn copy_command(&self) -> &'static str;
    fn replace_registry_container_hint(&self) -> &'static str;
    fn tag_error_entry(&self, tag: &str, err: &str) -> String;

    fn password_cannot_be_empty(&self) -> &'static str;
    fn encryption_error(&self, err: &str) -> String;
    fn incorrect_password_or_corrupt_config(&self) -> &'static str;
    fn incorrect_password_or_corrupt_config_error(&self, err: &str) -> String;
    fn failed_to_save_encryption_setup(&self, err: &str) -> String;
    fn configuration_cleared_restart(&self) -> &'static str;
    fn setup_encryption(&self) -> &'static str;
    fn unlock_configuration(&self) -> &'static str;
    fn setup_encryption_help(&self) -> &'static str;
    fn unlock_configuration_help(&self) -> &'static str;
    fn enter_password(&self) -> &'static str;
    fn set_key(&self) -> &'static str;
    fn unlock(&self) -> &'static str;
    fn reset_configuration(&self) -> &'static str;

    fn status_code(&self) -> &'static str;
    fn response_headers(&self) -> &'static str;
    fn response_body(&self) -> &'static str;
    fn curl_command(&self) -> &'static str;
    fn copy(&self) -> &'static str;
}

struct EnStrings;
struct ZhHansStrings;
struct ZhHantStrings;

static EN: EnStrings = EnStrings;
static ZH_HANS: ZhHansStrings = ZhHansStrings;
static ZH_HANT: ZhHantStrings = ZhHantStrings;

pub fn strings_for_locale(locale: Locale) -> &'static dyn Strings {
    match effective_locale(locale) {
        Locale::En | Locale::System => &EN,
        Locale::ZhHans => &ZH_HANS,
        Locale::ZhHant => &ZH_HANT,
    }
}

pub fn effective_locale(locale: Locale) -> Locale {
    match locale {
        Locale::System => detect_system_locale(),
        other => other,
    }
}

fn detect_system_locale() -> Locale {
    let system_locale = sys_locale::get_locale();
    map_system_locale(system_locale.as_deref())
}

fn map_system_locale(locale: Option<&str>) -> Locale {
    let Some(locale) = locale else {
        return Locale::En;
    };

    let normalized = locale.replace('_', "-").to_ascii_lowercase();

    if normalized.starts_with("zh-hant")
        || normalized.starts_with("zh-tw")
        || normalized.starts_with("zh-hk")
        || normalized.starts_with("zh-mo")
    {
        Locale::ZhHant
    } else if normalized.starts_with("zh-hans")
        || normalized.starts_with("zh-cn")
        || normalized.starts_with("zh-sg")
        || normalized == "zh"
    {
        Locale::ZhHans
    } else {
        Locale::En
    }
}

macro_rules! impl_strings {
    ($name:ident, {
        $($method:ident => $value:expr),+ $(,)?
    }, {
        $($fmt_method:ident($($arg:ident : $arg_ty:ty),*) => $fmt_value:expr),+ $(,)?
    }) => {
        impl Strings for $name {
            $(fn $method(&self) -> &'static str { $value })+
            $(fn $fmt_method(&self, $($arg: $arg_ty),*) -> String { format!($fmt_value, $($arg),*) })+
        }
    };
}

impl_strings!(EnStrings, {
    app_title => "Docker Registry Manager",
    ready => "Ready",
    no_selection => "No selection",
    selected_tag_context => "Selected tag context",
    selected_repository_context => "Selected repository context",
    selected_registry_context => "Selected registry context",
    context_guidance => "Choose a registry, repository, or tag to begin",
    refresh => "Refresh",
    settings => "Settings",
    close_settings => "Close Settings",
    open_settings => "Open Settings",
    settings_title => "Settings & Preferences",
    settings_subtitle => "Configure the workspace, console behavior, and registry manager defaults.",
    appearance => "Appearance",
    theme => "Theme",
    language => "Language",
    follow_system => "Follow System",
    light => "Light",
    dark => "Dark",
    english => "English",
    chinese_simplified => "Chinese (Simplified)",
    chinese_traditional => "Chinese (Traditional)",
    cache => "Cache",
    auto_refresh_interval => "Auto-refresh interval (seconds, 0 = disabled)",
    cache_max_age => "Cache max age (seconds)",
    reset_cache_settings => "Reset Cache Settings",
    import_export => "Import / Export",
    export_registries => "Export Registries",
    generate_export => "Generate Export",
    import_registries => "Import Registries (paste JSON)",
    paste_exported_json_here => "Paste exported JSON here...",
    import_action => "Import",
    registries => "Registries",
    registries_subtitle => "Select and manage your configured registries.",
    add_registry_button => "+ Add",
    filter_by_name_or_url => "Filter by name or URL",
    no_registries_configured => "No registries configured",
    no_registries_match_filter => "No registries match the current filter",
    delete_registry => "Delete Registry",
    delete_registry_warning => "This will remove the registry from your list. Your images on the registry will not be affected.",
    connected => "Connected",
    disconnected => "Disconnected",
    error => "Error",
    not_checked => "Not checked",
    edit_registry => "Edit Registry",
    add_registry => "Add Registry",
    name => "Name",
    url => "URL",
    authentication => "Authentication",
    anonymous => "Anonymous",
    basic_auth => "Basic Auth",
    bearer_token => "Bearer Token",
    username => "Username",
    password => "Password",
    token => "Token",
    cancel => "Cancel",
    save => "Save",
    registry_url_placeholder => "https://registry.example.com",
    repositories => "Repositories",
    repositories_subtitle => "Browse repositories in the current registry and choose what to inspect next.",
    select_registry_to_view_repositories => "Select a registry to view repositories",
    search_repositories => "Search repositories...",
    loading => "Loading...",
    retry => "Retry",
    no_repositories_found => "No repositories found",
    no_matching_repositories => "No matching repositories",
    delete_repository_title => "Delete repository",
    tags => "Tags",
    tags_subtitle => "Browse tags for the current repository and run filtering, selection, and bulk actions here.",
    select_repository_to_view_tags => "Select a repository to view tags",
    search_tags => "Search tags...",
    delete_selected => "Delete Selected",
    deleting => "Deleting...",
    no_tags_found => "No tags found",
    no_matching_tags => "No matching tags",
    delete_tags => "Delete Tags",
    action_cannot_be_undone => "This action cannot be undone.",
    delete_action => "Delete",
    manifest_details => "Manifest Details",
    select_tag_to_inspect => "Select a tag to inspect",
    select_tag_guidance => "Choose a tag from the list to review its manifest overview, image layers, and raw JSON details.",
    select_tag_body => "Start with a tag selection to populate the overview, inspect each layer, and optionally expand the raw manifest JSON.",
    loading_manifest => "Loading manifest...",
    overview => "Overview",
    tag => "Tag",
    digest => "Digest",
    media_type => "Media Type",
    total_size => "Total Size",
    raw_json => "Raw JSON",
    raw_json_help => "Expand the raw manifest payload only when you need the exact registry response.",
    show_raw_json => "Show Raw JSON",
    close => "Close",
    build_history => "Build History",
    no_history_available => "No history available",
    empty_layer => "empty layer",
    repository_deletion_completed => "Repository deletion completed.",
    deleting_tags => "Deleting tags...",
    delete_all_tags => "Delete All Tags",
    no_digest_returned => "No digest returned",
    empty_repository_hint => "This repository appears to be empty or contains only untagged manifests.",
    api_repository_delete_not_supported => "Docker Registry does not support direct repository deletion via API.",
    garbage_collect_hint => "To clean up, run garbage collection on the registry server:",
    copy_command => "Copy command",
    replace_registry_container_hint => "Replace <registry-container> with your registry container name.",
    password_cannot_be_empty => "Password cannot be empty.",
    incorrect_password_or_corrupt_config => "Incorrect password or corrupt configuration.",
    configuration_cleared_restart => "Configuration cleared. Please restart the application to set a new key.",
    setup_encryption => "Setup Encryption",
    unlock_configuration => "Unlock Configuration",
    setup_encryption_help => "Set a password to protect your registry credentials. This password will be required every time you start the app.",
    unlock_configuration_help => "Enter your password to decrypt your registry configurations.",
    enter_password => "Enter password...",
    set_key => "Set Key",
    unlock => "Unlock",
    reset_configuration => "Reset Configuration",
    status_code => "Status Code:",
    response_headers => "Response Headers",
    response_body => "Response Body",
    curl_command => "cURL Command:",
    copy => "Copy"
}, {
    context_label(path: &str) => "Context: {}",
    delete_registry_confirm(registry_name: &str) => "Are you sure you want to delete registry {}?",
    failed_to_fetch_repositories(err: &str) => "Failed to fetch repositories: {}",
    failed_to_create_client(err: &str) => "Failed to create client: {}",
    failed_to_fetch_tags(err: &str) => "Failed to fetch tags: {}",
    client_error(err: &str) => "Client error: {}",
    deleted_tags_successfully(deleted: usize) => "Deleted {} tags successfully",
    deleted_with_failures(deleted: usize, failed: usize) => "Deleted {}, {} failed",
    selected_count(count: usize) => "{} selected",
    deleted_tags(deleted: usize) => "Deleted {} tags",
    deleted_tags_with_errors(deleted: usize, errors: usize) => "Deleted {}, {} errors",
    error_with_details(err: &str) => "Error: {}",
    delete_tags_confirm(count: usize) => "Are you sure you want to delete {} tag(s)?",
    more_items(count: usize) => "...and {} more",
    loading_manifest_summary(tag: &str) => "Loading manifest metadata, layers, and raw JSON for the {} tag.",
    failed_manifest_summary(tag: &str) => "Unable to load manifest details for the {} tag right now.",
    pending_manifest_summary(tag: &str) => "Manifest details for the {} tag will appear here once data is available.",
    detail_summary(tag: &str, layers: usize) => "Review the {} manifest, including {} layers, media type, digest, and the raw payload when needed.",
    layers(count: usize) => "Layers ({})",
    failed_to_fetch_manifest(err: &str) => "Failed to fetch manifest: {}",
    deleted_of_tags(deleted: usize, total: usize) => "Deleted {} of {} tags.",
    failed_to_delete_tags(failed: usize) => "Failed to delete {} tags:",
    delete_repository_confirm(repo_name: &str) => "Are you sure you want to delete repository {}?",
    delete_all_tags_warning(total_tags: usize) => "This will delete all {} tags. This action cannot be undone.",
    repository_has_no_tags(repo_name: &str) => "Repository {} has no tags.",
    tag_error_entry(tag: &str, err: &str) => "{}: {}",
    encryption_error(err: &str) => "Encryption error: {}",
    incorrect_password_or_corrupt_config_error(err: &str) => "Incorrect password or corrupt configuration. Error: {}",
    failed_to_save_encryption_setup(err: &str) => "Failed to save encryption setup. Error: {}"
});

impl_strings!(ZhHansStrings, {
    app_title => "Docker Registry Manager",
    ready => "就绪",
    no_selection => "未选择",
    selected_tag_context => "当前为标签上下文",
    selected_repository_context => "当前为仓库上下文",
    selected_registry_context => "当前为仓库源上下文",
    context_guidance => "请选择仓库源、仓库或标签开始操作",
    refresh => "刷新",
    settings => "设置",
    close_settings => "关闭设置",
    open_settings => "打开设置",
    settings_title => "设置与偏好",
    settings_subtitle => "配置工作区、控制台行为以及镜像仓库管理器默认项。",
    appearance => "外观",
    theme => "主题",
    language => "语言",
    follow_system => "跟随系统",
    light => "浅色",
    dark => "深色",
    english => "English",
    chinese_simplified => "中文简体",
    chinese_traditional => "中文繁体",
    cache => "缓存",
    auto_refresh_interval => "自动刷新间隔（秒，0 表示禁用）",
    cache_max_age => "缓存最大时长（秒）",
    reset_cache_settings => "重置缓存设置",
    import_export => "导入 / 导出",
    export_registries => "导出仓库源",
    generate_export => "生成导出内容",
    import_registries => "导入仓库源（粘贴 JSON）",
    paste_exported_json_here => "在此粘贴导出的 JSON...",
    import_action => "导入",
    registries => "仓库源",
    registries_subtitle => "选择并管理已配置的仓库源。",
    add_registry_button => "+ 添加",
    filter_by_name_or_url => "按名称或 URL 过滤",
    no_registries_configured => "尚未配置仓库源",
    no_registries_match_filter => "没有匹配当前过滤条件的仓库源",
    delete_registry => "删除仓库源",
    delete_registry_warning => "这会将该仓库源从列表中移除，但不会影响仓库源上的镜像。",
    connected => "已连接",
    disconnected => "未连接",
    error => "错误",
    not_checked => "未检查",
    edit_registry => "编辑仓库源",
    add_registry => "添加仓库源",
    name => "名称",
    url => "URL",
    authentication => "认证方式",
    anonymous => "匿名",
    basic_auth => "基础认证",
    bearer_token => "Bearer Token",
    username => "用户名",
    password => "密码",
    token => "令牌",
    cancel => "取消",
    save => "保存",
    registry_url_placeholder => "https://registry.example.com",
    repositories => "仓库",
    repositories_subtitle => "浏览当前仓库源中的仓库，并选择后续要查看的对象。",
    select_registry_to_view_repositories => "请选择仓库源以查看仓库",
    search_repositories => "搜索仓库...",
    loading => "加载中...",
    retry => "重试",
    no_repositories_found => "未找到仓库",
    no_matching_repositories => "没有匹配的仓库",
    delete_repository_title => "删除仓库",
    tags => "标签",
    tags_subtitle => "查看当前仓库的标签，并在此执行筛选、选择和批量操作。",
    select_repository_to_view_tags => "请选择仓库以查看标签",
    search_tags => "搜索标签...",
    delete_selected => "删除所选",
    deleting => "删除中...",
    no_tags_found => "未找到标签",
    no_matching_tags => "没有匹配的标签",
    delete_tags => "删除标签",
    action_cannot_be_undone => "此操作无法撤销。",
    delete_action => "删除",
    manifest_details => "Manifest 详情",
    select_tag_to_inspect => "选择要查看的标签",
    select_tag_guidance => "从列表中选择一个标签，以查看其 manifest 概览、镜像层和原始 JSON 详情。",
    select_tag_body => "先选择一个标签，即可在这里查看概览、检查各层，并按需展开原始 manifest JSON。",
    loading_manifest => "正在加载 manifest...",
    overview => "概览",
    tag => "标签",
    digest => "摘要",
    media_type => "媒体类型",
    total_size => "总大小",
    raw_json => "原始 JSON",
    raw_json_help => "仅在需要查看仓库源返回的精确内容时展开原始 manifest。",
    show_raw_json => "显示原始 JSON",
    close => "关闭",
    build_history => "构建历史",
    no_history_available => "暂无历史记录",
    empty_layer => "空层",
    repository_deletion_completed => "仓库删除完成。",
    deleting_tags => "正在删除标签...",
    delete_all_tags => "删除全部标签",
    no_digest_returned => "未返回摘要",
    empty_repository_hint => "该仓库看起来为空，或只包含未打标签的 manifest。",
    api_repository_delete_not_supported => "Docker Registry API 不支持直接删除仓库。",
    garbage_collect_hint => "如需清理，请在仓库源服务器上执行垃圾回收：",
    copy_command => "复制命令",
    replace_registry_container_hint => "请将 <registry-container> 替换为你的仓库源容器名称。",
    password_cannot_be_empty => "密码不能为空。",
    incorrect_password_or_corrupt_config => "密码错误或配置已损坏。",
    configuration_cleared_restart => "配置已清除。请重启应用后再设置新密钥。",
    setup_encryption => "设置加密",
    unlock_configuration => "解锁配置",
    setup_encryption_help => "设置一个密码来保护你的仓库凭据。每次启动应用时都需要输入该密码。",
    unlock_configuration_help => "输入密码以解密你的仓库配置。",
    enter_password => "输入密码...",
    set_key => "设置密钥",
    unlock => "解锁",
    reset_configuration => "重置配置",
    status_code => "状态码：",
    response_headers => "响应头",
    response_body => "响应体",
    curl_command => "cURL 命令：",
    copy => "复制"
}, {
    context_label(path: &str) => "上下文：{}",
    delete_registry_confirm(registry_name: &str) => "确认删除仓库源 {} 吗？",
    failed_to_fetch_repositories(err: &str) => "获取仓库失败：{}",
    failed_to_create_client(err: &str) => "创建客户端失败：{}",
    failed_to_fetch_tags(err: &str) => "获取标签失败：{}",
    client_error(err: &str) => "客户端错误：{}",
    deleted_tags_successfully(deleted: usize) => "已成功删除 {} 个标签",
    deleted_with_failures(deleted: usize, failed: usize) => "已删除 {} 个，失败 {} 个",
    selected_count(count: usize) => "已选择 {} 项",
    deleted_tags(deleted: usize) => "已删除 {} 个标签",
    deleted_tags_with_errors(deleted: usize, errors: usize) => "已删除 {} 个，错误 {} 个",
    error_with_details(err: &str) => "错误：{}",
    delete_tags_confirm(count: usize) => "确认删除 {} 个标签吗？",
    more_items(count: usize) => "...以及另外 {} 项",
    loading_manifest_summary(tag: &str) => "正在加载标签 {} 的 manifest 元数据、镜像层和原始 JSON。",
    failed_manifest_summary(tag: &str) => "当前无法加载标签 {} 的 manifest 详情。",
    pending_manifest_summary(tag: &str) => "标签 {} 的 manifest 详情将在数据可用后显示在这里。",
    detail_summary(tag: &str, layers: usize) => "查看标签 {} 的 manifest，包括 {} 层、媒体类型、摘要以及按需查看的原始内容。",
    layers(count: usize) => "镜像层（{}）",
    failed_to_fetch_manifest(err: &str) => "获取 manifest 失败：{}",
    deleted_of_tags(deleted: usize, total: usize) => "已删除 {}/{} 个标签。",
    failed_to_delete_tags(failed: usize) => "有 {} 个标签删除失败：",
    delete_repository_confirm(repo_name: &str) => "确认删除仓库 {} 吗？",
    delete_all_tags_warning(total_tags: usize) => "这将删除全部 {} 个标签。此操作无法撤销。",
    repository_has_no_tags(repo_name: &str) => "仓库 {} 没有标签。",
    tag_error_entry(tag: &str, err: &str) => "{}：{}",
    encryption_error(err: &str) => "加密错误：{}",
    incorrect_password_or_corrupt_config_error(err: &str) => "密码错误或配置已损坏。错误：{}",
    failed_to_save_encryption_setup(err: &str) => "保存加密设置失败。错误：{}"
});

impl_strings!(ZhHantStrings, {
    app_title => "Docker Registry Manager",
    ready => "就緒",
    no_selection => "未選擇",
    selected_tag_context => "目前為標籤上下文",
    selected_repository_context => "目前為倉庫上下文",
    selected_registry_context => "目前為倉庫來源上下文",
    context_guidance => "請選擇倉庫來源、倉庫或標籤開始操作",
    refresh => "重新整理",
    settings => "設定",
    close_settings => "關閉設定",
    open_settings => "開啟設定",
    settings_title => "設定與偏好",
    settings_subtitle => "設定工作區、主控台行為以及鏡像倉庫管理器的預設值。",
    appearance => "外觀",
    theme => "主題",
    language => "語言",
    follow_system => "跟隨系統",
    light => "淺色",
    dark => "深色",
    english => "English",
    chinese_simplified => "中文簡體",
    chinese_traditional => "中文繁體",
    cache => "快取",
    auto_refresh_interval => "自動重新整理間隔（秒，0 表示停用）",
    cache_max_age => "快取最長保留時間（秒）",
    reset_cache_settings => "重設快取設定",
    import_export => "匯入 / 匯出",
    export_registries => "匯出倉庫來源",
    generate_export => "產生匯出內容",
    import_registries => "匯入倉庫來源（貼上 JSON）",
    paste_exported_json_here => "在此貼上匯出的 JSON...",
    import_action => "匯入",
    registries => "倉庫來源",
    registries_subtitle => "選擇並管理已設定的倉庫來源。",
    add_registry_button => "+ 新增",
    filter_by_name_or_url => "依名稱或 URL 篩選",
    no_registries_configured => "尚未設定倉庫來源",
    no_registries_match_filter => "沒有符合目前篩選條件的倉庫來源",
    delete_registry => "刪除倉庫來源",
    delete_registry_warning => "這會從清單中移除此倉庫來源，但不會影響倉庫來源上的映像。",
    connected => "已連線",
    disconnected => "未連線",
    error => "錯誤",
    not_checked => "未檢查",
    edit_registry => "編輯倉庫來源",
    add_registry => "新增倉庫來源",
    name => "名稱",
    url => "URL",
    authentication => "驗證方式",
    anonymous => "匿名",
    basic_auth => "基本驗證",
    bearer_token => "Bearer Token",
    username => "使用者名稱",
    password => "密碼",
    token => "權杖",
    cancel => "取消",
    save => "儲存",
    registry_url_placeholder => "https://registry.example.com",
    repositories => "倉庫",
    repositories_subtitle => "瀏覽目前倉庫來源中的倉庫，並選擇接下來要查看的項目。",
    select_registry_to_view_repositories => "請先選擇倉庫來源以查看倉庫",
    search_repositories => "搜尋倉庫...",
    loading => "載入中...",
    retry => "重試",
    no_repositories_found => "找不到倉庫",
    no_matching_repositories => "沒有符合的倉庫",
    delete_repository_title => "刪除倉庫",
    tags => "標籤",
    tags_subtitle => "查看目前倉庫的標籤，並在此進行篩選、選取與批次操作。",
    select_repository_to_view_tags => "請先選擇倉庫以查看標籤",
    search_tags => "搜尋標籤...",
    delete_selected => "刪除所選",
    deleting => "刪除中...",
    no_tags_found => "找不到標籤",
    no_matching_tags => "沒有符合的標籤",
    delete_tags => "刪除標籤",
    action_cannot_be_undone => "此操作無法復原。",
    delete_action => "刪除",
    manifest_details => "Manifest 詳情",
    select_tag_to_inspect => "選擇要檢視的標籤",
    select_tag_guidance => "從清單中選擇一個標籤，以查看其 manifest 概覽、映像層與原始 JSON 詳情。",
    select_tag_body => "先選擇一個標籤，即可在這裡查看概覽、檢查各層，並依需要展開原始 manifest JSON。",
    loading_manifest => "正在載入 manifest...",
    overview => "概覽",
    tag => "標籤",
    digest => "摘要",
    media_type => "媒體類型",
    total_size => "總大小",
    raw_json => "原始 JSON",
    raw_json_help => "只有在需要查看倉庫來源回傳的精確內容時才展開原始 manifest。",
    show_raw_json => "顯示原始 JSON",
    close => "關閉",
    build_history => "建置歷史",
    no_history_available => "沒有歷史紀錄",
    empty_layer => "空層",
    repository_deletion_completed => "倉庫刪除完成。",
    deleting_tags => "正在刪除標籤...",
    delete_all_tags => "刪除全部標籤",
    no_digest_returned => "未回傳摘要",
    empty_repository_hint => "此倉庫看起來是空的，或僅包含未標記的 manifest。",
    api_repository_delete_not_supported => "Docker Registry API 不支援直接刪除倉庫。",
    garbage_collect_hint => "若要清理，請在倉庫來源伺服器上執行垃圾回收：",
    copy_command => "複製命令",
    replace_registry_container_hint => "請將 <registry-container> 替換為你的倉庫來源容器名稱。",
    password_cannot_be_empty => "密碼不得為空。",
    incorrect_password_or_corrupt_config => "密碼錯誤或設定已損毀。",
    configuration_cleared_restart => "設定已清除。請重新啟動應用程式後再設定新金鑰。",
    setup_encryption => "設定加密",
    unlock_configuration => "解鎖設定",
    setup_encryption_help => "設定一組密碼來保護你的倉庫憑證。每次啟動應用程式時都需要輸入此密碼。",
    unlock_configuration_help => "輸入密碼以解密你的倉庫設定。",
    enter_password => "輸入密碼...",
    set_key => "設定金鑰",
    unlock => "解鎖",
    reset_configuration => "重設設定",
    status_code => "狀態碼：",
    response_headers => "回應標頭",
    response_body => "回應內容",
    curl_command => "cURL 命令：",
    copy => "複製"
}, {
    context_label(path: &str) => "上下文：{}",
    delete_registry_confirm(registry_name: &str) => "確認刪除倉庫來源 {} 嗎？",
    failed_to_fetch_repositories(err: &str) => "取得倉庫失敗：{}",
    failed_to_create_client(err: &str) => "建立客戶端失敗：{}",
    failed_to_fetch_tags(err: &str) => "取得標籤失敗：{}",
    client_error(err: &str) => "客戶端錯誤：{}",
    deleted_tags_successfully(deleted: usize) => "已成功刪除 {} 個標籤",
    deleted_with_failures(deleted: usize, failed: usize) => "已刪除 {} 個，失敗 {} 個",
    selected_count(count: usize) => "已選取 {} 項",
    deleted_tags(deleted: usize) => "已刪除 {} 個標籤",
    deleted_tags_with_errors(deleted: usize, errors: usize) => "已刪除 {} 個，錯誤 {} 個",
    error_with_details(err: &str) => "錯誤：{}",
    delete_tags_confirm(count: usize) => "確認刪除 {} 個標籤嗎？",
    more_items(count: usize) => "...以及另外 {} 項",
    loading_manifest_summary(tag: &str) => "正在載入標籤 {} 的 manifest 中繼資料、映像層與原始 JSON。",
    failed_manifest_summary(tag: &str) => "目前無法載入標籤 {} 的 manifest 詳情。",
    pending_manifest_summary(tag: &str) => "標籤 {} 的 manifest 詳情會在資料可用後顯示於此。",
    detail_summary(tag: &str, layers: usize) => "查看標籤 {} 的 manifest，包括 {} 層、媒體類型、摘要，以及需要時可查看的原始內容。",
    layers(count: usize) => "映像層（{}）",
    failed_to_fetch_manifest(err: &str) => "取得 manifest 失敗：{}",
    deleted_of_tags(deleted: usize, total: usize) => "已刪除 {}/{} 個標籤。",
    failed_to_delete_tags(failed: usize) => "有 {} 個標籤刪除失敗：",
    delete_repository_confirm(repo_name: &str) => "確認刪除倉庫 {} 嗎？",
    delete_all_tags_warning(total_tags: usize) => "這將刪除全部 {} 個標籤。此操作無法復原。",
    repository_has_no_tags(repo_name: &str) => "倉庫 {} 沒有標籤。",
    tag_error_entry(tag: &str, err: &str) => "{}：{}",
    encryption_error(err: &str) => "加密錯誤：{}",
    incorrect_password_or_corrupt_config_error(err: &str) => "密碼錯誤或設定已損毀。錯誤：{}",
    failed_to_save_encryption_setup(err: &str) => "儲存加密設定失敗。錯誤：{}"
});

#[cfg(test)]
mod tests {
    use super::{effective_locale, map_system_locale, strings_for_locale};
    use crate::models::Locale;

    #[test]
    fn system_locale_maps_simplified_variants() {
        assert_eq!(map_system_locale(Some("zh-CN")), Locale::ZhHans);
        assert_eq!(map_system_locale(Some("zh_Hans")), Locale::ZhHans);
        assert_eq!(map_system_locale(Some("zh")), Locale::ZhHans);
    }

    #[test]
    fn system_locale_maps_traditional_variants() {
        assert_eq!(map_system_locale(Some("zh-TW")), Locale::ZhHant);
        assert_eq!(map_system_locale(Some("zh-Hant-HK")), Locale::ZhHant);
    }

    #[test]
    fn unsupported_system_locale_falls_back_to_english() {
        assert_eq!(map_system_locale(Some("en-US")), Locale::En);
        assert_eq!(map_system_locale(None), Locale::En);
        assert_eq!(effective_locale(Locale::En), Locale::En);
    }

    #[test]
    fn locale_uses_expected_static_copy() {
        assert_eq!(strings_for_locale(Locale::ZhHans).language(), "语言");
        assert_eq!(strings_for_locale(Locale::ZhHant).language(), "語言");
        assert_eq!(strings_for_locale(Locale::En).language(), "Language");
    }
}
