// TODO: Port to SSE branch
std::string GetOsStringAsUtf8(std::wstring in) {
    using convert_type = std::codecvt_utf8<wchar_t>;
    std::wstring_convert<convert_type, wchar_t> converter;
    std::string converted_str = converter.to_bytes( in );
    return converted_str;
}

std::string GetLogFile() {
	auto path = F4SE::log::log_directory();
    if (!path) {
        return "";
    }
    std::optional<std::string> utf8Path = GetOsStringAsUtf8(path->wstring());
    if (!utf8Path.has_value()) {
        return "";
    }
    return std::format("{}\\{}.log", utf8Path.value(), Version::PROJECT.data());
}