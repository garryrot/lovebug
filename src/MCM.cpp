
using DevicePageStruct = RE::BSScript::structure_wrapper<"Telekinesis", "DevicePage">;
DevicePageStruct DataToStruct(DevicePage data)
{
    DevicePageStruct pack;
    pack.insert("Index", data.index);
    if (data.index >= 0)
    {
        pack.insert("Actuator", (std::string) data.actuator);
        pack.insert("Enabled", data.enabled );
        pack.insert("Anal", data.anal );
        pack.insert("Clitoral", data.clitoral );
        pack.insert("Nipple", data.nipple );
        pack.insert("Oral", data.oral );
        pack.insert("Penis", data.penis );
        pack.insert("Vaginal", data.vaginal );
    }
    return pack;
}

DevicePage StructToData(DevicePageStruct pack) {
    DevicePage data;
    data.index = pack.find<int32_t>("Index", false).value();
    data.actuator = pack.find<std::string>("Actuator", false).value(); 
    data.enabled = pack.find<bool>("Enabled", false).value();
    data.anal = pack.find<bool>("Anal", false).value();
    data.clitoral = pack.find<bool>("Clitoral", false).value();
    data.nipple = pack.find<bool>("Nipple", false).value();
    data.oral = pack.find<bool>("Oral", false).value();
    data.penis = pack.find<bool>("Penis", false).value();
    data.vaginal = pack.find<bool>("Vaginal", false).value();
    return data;
}

DevicePageStruct MCM_Devices_Get(std::monostate, int index)
{
    return DataToStruct(lb_actuator_get(index));
}

bool MCM_Devices_Set(std::monostate, DevicePageStruct data)
{
    return lb_actuator_set( StructToData(data) );
}

uint32_t MCM_Devices_Len(std::monostate)
{
    return lb_actuator_len();
}
