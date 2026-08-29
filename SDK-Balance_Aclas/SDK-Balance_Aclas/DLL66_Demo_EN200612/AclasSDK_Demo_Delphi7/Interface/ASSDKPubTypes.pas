{ *******************************************************

  Aclas SDK -- 定义

  Copyright (C) 2014 Aclas. All rights reserved.

  ******************************************************* }
// ------------------------------------------------------------------------------
// 2014-04-26: GLL(6355) Create
// ------------------------------------------------------------------------------

unit ASSDKPubTypes;

interface

uses
  Windows;

const
  // 时间格式
  ASSDK_DateTimeFormat = 'YYYY-MM-DD HH:mm:SS';
  // 任务失败
  ASSDK_Task_Failed = THandle(-1);
  // 删除全部的文件名
  ASSDK_FileName_All: string = '*';
  { 错误信息 }
  // 正常
  ASSDK_Err_Success = $0;
  // 进度事件
  ASSDK_Err_Progress = $1;
  // 手动停止
  ASSDK_Err_Terminate = $2;
  { 动态库错误事件 }
  // 已初始化
  ASSDK_Err_AlreadyInitialize = $100;
  // 未初始化
  ASSDK_Err_NotInitialize = $101;
  // 设备不存在
  ASSDK_Err_DeviceNotExists = $102;
  // 不支持的协议类型
  ASSDK_Err_ProtocolTypeNotSupport = $103;
  // 该数据类型不支持此操作
  ASSDK_Err_DataTypeNotSupportProc = $104;
  // 该数据类型不支持
  ASSDK_Err_DataTypeNotSupport = $105;
  // 无法打开输入文件
  ASSDK_Err_CannotOpenInputFile = $108;
  // 字段数与内容数不匹配
  ASSDK_Err_ImportDataFailed = $109;
  // 通讯数据异常
  ASSDK_Err_CommDataFailed = $10A;
  // 解析数据异常
  ASSDK_Err_ParseDataFailed = $10B;
  // CodePage错误
  ASSDK_Err_InvalidCodePage = $10C;
  // 无法创建输出文件
  ASSDK_Err_CannotCreateOutputFile = $10D;
  { 下位机错误 }
  // 下位机错误基数
  ASSDK_Err_DeviceError = $10000;

  { 常量 }
  // 在线销售默认端口
  ASSDK_OnlineSell_DefaultPort = 8001;

  // CodePage
  ASSDK_CodePage_AnsiDef = 0;
  ASSDK_CodePage_Unicode = 1200;
  ASSDK_CodePage_BigEndianUnicode = 1201;
  ASSDK_CodePage_UTF8 = 65001;

  // ProtocolType
  ASSDK_ProtocolType_None = 0;
  ASSDK_ProtocolType_Pecr = 1;
  ASSDK_ProtocolType_Hecr = 2;
  ASSDK_ProtocolType_TSecr = 3;

  // ProcType
  ASSDK_ProcType_Down = 0;
  ASSDK_ProcType_UP = 1;
  ASSDK_ProcType_Edit = 2;
  ASSDK_ProcType_Del = 3;
  ASSDK_ProcType_List = 4;
  ASSDK_ProcType_Reserve = $10;

  // DataType
  ASSDK_DataType_Nil = $FFFFFFFF;
  ASSDK_DataType_PLU = $0000;
  ASSDK_DataType_Unit = $0001;
  ASSDK_DataType_Department = $0002;
  ASSDK_DataType_HotKey = $0003;
  ASSDK_DataType_Group = $0004;
  ASSDK_DataType_Discount = $0005;
  ASSDK_DataType_Origin = $0006;
  ASSDK_DataType_Country = $0007;
  ASSDK_DataType_SlaughterHouse = $0008;
  ASSDK_DataType_Cuttinghall = $0009;
  ASSDK_DataType_Tare = $000A;
  ASSDK_DataType_Nutrition = $000B;
  ASSDK_DataType_Note1 = $000C;
  ASSDK_DataType_Note2 = $000D;
  ASSDK_DataType_Note3 = $000E;
  // ASSDK_DataType_TextMessage = $000F;
  ASSDK_DataType_Options = $0010;
  ASSDK_DataType_CustomBarcode = $0011;
  ASSDK_DataType_LabelPrintRecord = $0012;
  ASSDK_DataType_HeaderInfo = $0013;
  ASSDK_DataType_FooterInfo = $0014;
  ASSDK_DataType_AdvertisementInfo = $0015;
  ASSDK_DataType_HeaderLogo = $0016;
  ASSDK_DataType_FooterLogo = $0017;
  ASSDK_DataType_LabelAdvertisement = $0018;
  ASSDK_DataType_VendorInfo = $0019;
  ASSDK_DataType_NutritionElement = $001A;
  ASSDK_DataType_NutritionInfo = $001B;
  ASSDK_DataType_Note4 = $001C;
  ASSDK_DataType_PosMain = $001D; // 销售主档纪录
  ASSDK_DataType_PosItem = $001E; // 销售明细纪录
  ASSDK_DataType_PosRecItem = $001F; // 销售收款纪录

  //
  ASSDK_DataType_Trace1 = $0101;
  ASSDK_DataType_TraceMap1 = $0102;
  ASSDK_DataType_Trace2 = $0103;
  ASSDK_DataType_TraceMap2 = $0104;
  ASSDK_DataType_Message1 = $0105;
  ASSDK_DataType_Message2 = $0106;
  ASSDK_DataType_SaleRecord = $0107;
  ASSDK_DataType_PLUReport = $0108;
  ASSDK_DataType_GroupReport = $0109;
  ASSDK_DataType_DepartmentReport = $010A;

  { File }
  ASSDK_DataType_Image = $1000;
  ASSDK_DataType_Video = $1001;
  ASSDK_DataType_Label = $1002;
  ASSDK_DataType_File = $1003;
  ASSDK_DataType_DisplayImage = $1004;
  ASSDK_DataType_KeyImage = $1005;
  ASSDK_DataType_CategoryImage = $1006;
  ASSDK_DataType_DataBase = $1007;
  ASSDK_DataType_Parameter = $1008;
  ASSDK_DataType_DataBaseRecovery = $1009;
  { System }
  ASSDK_DataType_Time = $2001;
  ASSDK_DataType_Font = $2002;
  ASSDK_DataType_Firmware = $2003;

  ASSDK_DataType_PrnDensity = $2004; // 打印浓度
  ASSDK_DataType_LcdDensity = $2005; // LCD对比度
  ASSDK_DataType_PrnCompensate = $2006; // 温度补偿
  { }
  { Cash Register(收银机) }
  ASSDK_DataType_CR_CurComm = $3000;
  ASSDK_DataType_CR_CommType = $3001;
  ASSDK_DataType_CR_TmpBuy1Give1Sale = $3002;
  ASSDK_DataType_CR_Member = $3003;
  ASSDK_DataType_CR_SaleScheduleItem = $3004;
  ASSDK_DataType_CR_MiCommQtySale = $3005;
  ASSDK_DataType_CR_CommGroup = $3006;
  ASSDK_DataType_CR_MemCardLevel = $3007;
  ASSDK_DataType_CR_PayName = $3008;
  ASSDK_DataType_CR_Printer = $3009;
  ASSDK_DataType_CR_Taste = $300A;
  ASSDK_DataType_CR_Place = $300B;
  ASSDK_DataType_CR_Table = $300C;
  ASSDK_DataType_CR_Layby = $300D;
  ASSDK_DataType_CR_CusType = $300E;
  ASSDK_DataType_CR_Unit = $300F;
  ASSDK_DataType_CR_Emp = $3010;
  ASSDK_DataType_CR_VoidReson = $3011;
  ASSDK_DataType_CR_SetMeal = $3012;
  ASSDK_DataType_CR_RfidBlack = $3013;
  ASSDK_DataType_CR_RfidCarASSDK_DataType_ype = $3014;
  ASSDK_DataType_CR_RfidPromo = $3015;
  ASSDK_DataType_CR_RfidStoreAmtCtl = $3016;
  ASSDK_DataType_CR_EmpAuth = $3017;
  ASSDK_DataType_CR_MemIntegral = $3018;
  ASSDK_DataType_CR_CommTypeMemberRate = $3019;
  ASSDK_DataType_CR_Arm_extbCSPosMain = $301A;
  ASSDK_DataType_CR_Arm_extbCSPosItem = $301B;
  ASSDK_DataType_CR_Arm_extbCSPosRecItem = $301C;
  ASSDK_DataType_CR_Arm_extbCSMiMemCardIO = $301D;
  ASSDK_DataType_CR_Arm_extbCSAttendence = $301E;
  ASSDK_DataType_CR_Arm_extbCsCashInOut = $301F;
  ASSDK_DataType_CR_Arm_extbCsLaybyTb = $3020; // 会员签单表
  ASSDK_DataType_CR_tmpTypeGroupTb = $3021; // 类别分组
  ASSDK_DataType_CR_tmpTaxIndexTb = $3022; // 税率设置
  ASSDK_DataType_CR_tmpItemGroup = $3023; // 分组表
  ASSDK_DataType_CR_tmpIIGrpTb = $3024; // 套餐分组表
  ASSDK_DataType_CR_tmpInstructionTb = $3025; // 做法表
  ASSDK_DataType_CR_tmpVirtualCommTypeTb = $3026; // 虚拟类别
  ASSDK_DataType_CR_tmpPromotionPlanTb = $3027; // 促销方案主表
  ASSDK_DataType_CR_tmpPromotionPlanmTb = $3028; // 促销方案明细表
  ASSDK_DataType_CR_tmpPromotionPlanGiftTb = $3029; // 促销方案明细赠品表
  ASSDK_DataType_CR_tmpBoothTb = $302A; // 摊位档案
  ASSDK_DataType_CR_DailyReportTb = $302B; // 营业日报
  ASSDK_DataType_CR_OpenDrawerTb = $302C; // 钱箱
  ASSDK_DataType_CR_tmpMemberDiscountTb = $302D; // 会员区间折扣

type
{$Z4}
  TASSDKUserStorageType = (ustRead = 0, ustWrite = 1, ustPrivateRead = $10, ustPrivateWrite = $11);
{$Z-}
  // 16字节字符串(编码规则根据TASSDKEncoding类型而定)
  TASSDKBytes16 = array [0 .. 15] of Byte;
{$Z4}
  // 数据类型
  TASSDKDataType = ( //
    { Data }
    dtPLU = $0000, //
    dtUnit = $0001, //
    dtDepartment = $0002, //
    dtHotKey = $0003, //
    dtGroup = $0004, //
    dtDiscount = $0005, //
    dtOrigin = $0006, //
    dtCountry = $0007, //
    dtSlaughterHouse = $0008, //
    dtCuttinghall = $0009, //
    dtTare = $000A, //
    dtNutrition = $000B, //
    dtNote1 = $000C, //
    dtNote2 = $000D, //
    dtNote3 = $000E, //
    // dtTextMessage = $000F, //
    dtOptions = $0010, //
    dtCustomBarcode = $0011, //
    dtLabelPrintRecord = $0012, //
    dtHeaderInfo = $0013, //
    dtFooterInfo = $0014, //
    dtAdvertisementInfo = $0015, //
    dtHeaderLogo = $0016, //
    dtFooterLogo = $0017, //
    dtLabelAdvertisement = $0018, //
    dtVendorInfo = $0019, //
    dtNutritionElement = $001A, //
    dtNutritionInfo = $001B, //
    dtNote4 = $001C, //
    dtPosMain = $001D, // 销售主档纪录
    dtPosItem = $001E, // 销售明细纪录
    dtPosRecItem = $001F, // 销售收款纪录
    //
    dtTrace1 = $0101, //
    dtTraceMap1 = $0102, //
    dtTrace2 = $0103, //
    dtTraceMap2 = $0104, //
    dtMessage1 = $0105, //
    dtMessage2 = $0106, //
    dtSaleRecord = $0107, // 流水账
    dtPLUReport = $0108, //
    dtGroupReport = $0109, //
    dtDepartmentReport = $010A, //
    { File }
    dtImage = $1000, //
    dtVideo = $1001, //
    dtLabel = $1002, //
    dtFile = $1003, //
    dtDisplayImage = $1004, //
    dtKeyImage = $1005, //
    dtCategoryImage = $1006, //
    dtDataBase = $1007, //
    dtParameter = $1008, //
    dtDataBaseRecovery = $1009, //
    dtFileLimit = $1FFF, //
    { System }
    dtTime = $2001, //
    dtFont = $2002, //
    dtFirmware = $2003, //
    dtPrnDensity = $2004, // 打印浓度
    dtLcdDensity = $2005, // Lcd对比度
    dtPrnCompensate = $2006, // 温度补偿
    { }
    { Cash Register(收银机) }
    dtCR_CurComm = $3000, // 商品档案
    dtCR_CommType = $3001, // 商品类别
    dtCR_TmpBuy1Give1Sale = $3002, // 买一送一
    dtCR_Member = $3003, // 会员表
    dtCR_SaleScheduleItem = $3004, // 促销排程
    dtCR_MiCommQtySale = $3005, //
    dtCR_CommGroup = $3006, //
    dtCR_MemCardLevel = $3007, //
    dtCR_PayName = $3008, //
    dtCR_Printer = $3009, // 打印机设置
    dtCR_Taste = $300A, // 口味
    dtCR_Place = $300B, // 场地
    dtCR_Table = $300C, // 餐桌
    dtCR_Layby = $300D, // 会员签单表
    dtCR_CusType = $300E, // 顾客类型
    dtCR_Unit = $300F, // 计量单位
    dtCR_Emp = $3010, // 职员资料
    dtCR_VoidReson = $3011, // 退菜原因
    dtCR_SetMeal = $3012, // 套餐表
    dtCR_RfidBlack = $3013, // 黑卡
    dtCR_RfidCardType = $3014, // Rfid卡等级
    dtCR_RfidPromo = $3015, // Rfid促销
    dtCR_RfidStoreAmtCtl = $3016, // Rfid储值金额
    dtCR_EmpAuth = $3017, // 权限组别定义
    dtCR_MemIntegral = $3018, // 会员兑换积分列表
    dtCR_CommTypeMemberRate = $3019, // 类别会员扣率表
    dtCR_Arm_extbCSPosMain = $301A, // 销售主档
    dtCR_Arm_extbCSPosItem = $301B, // 销售明细档
    dtCR_Arm_extbCSPosRecItem = $301C, // 销售付款单
    dtCR_Arm_extbCSMiMemCardIO = $301D, // 卡流水
    dtCR_Arm_extbCSAttendence = $301E, // 考勤表
    dtCR_Arm_extbCsCashInOut = $301F, // 现金入出档
    dtCR_Arm_extbCsLaybyTb = $3020, // 会员签单表
    dtCR_tmpTypeGroupTb = $3021, // 类别分组
    dtCR_tmpTaxIndexTb = $3022, // 税率设置
    dtCR_tmpItemGroup = $3023, // 分组表
    dtCR_tmpIIGrpTb = $3024, // 套餐分组表
    dtCR_tmpInstructionTb = $3025, // 做法表
    dtCR_tmpVirtualCommTypeTb = $3026, // 虚拟类别
    dtCR_tmpPromotionPlanTb = $3027, // 促销方案主表
    dtCR_tmpPromotionPlanmTb = $3028, // 促销方案明细表
    dtCR_tmpPromotionPlanGiftTb = $3029, // 促销方案明细赠品表
    dtCR_tmpBoothTb = $302A, // 摊位档案
    dtCR_DailyReportTb = $302B, // 营业日报
    dtCR_OpenDrawerTb = $302C, // 钱箱
    dtCR_tmpMemberDiscountTb = $302D // 会员区间折扣

    // dtNil = $FFFF //
    );

{$Z-}
  type UInt32 = Cardinal;
  type UInt8 = byte;
  type UInt16 = Word;

  // 设备信息
  PASSDKDeviceInfo = ^TASSDKDeviceInfo;

  TASSDKDeviceInfo = packed record
    ProtocolType: UInt32; // 协议类型
    Addr: UInt32; // 地址
    Port: UInt32; // 端口
    Name: TASSDKBytes16; // 设备名(Ansi)
    ID: UInt32; // 设备ID(Ansi)
    Version: UInt32; // 软件版本
    Country: UInt8; // 国家类别
    DepartmentID: UInt8; // 部门号
    KeyType: UInt8; // 键盘类型
    PrinterDot: UInt64; // 打印头已打印公里点数
    PrnStartDate: TDateTime; // 打印头起用时间 YYYYMMDDhhmmss bin// 20 13 11 18 15 07 00
    LabelPage: UInt32; // 打印标签张数
    PrinterNo: UInt32; // 打印头序列号
    PLUStorage: UInt16; // PLU可存储数量
    HotKeyCount: UInt16; // 支持热键数量
    NutritionStorage: UInt16; // 营养信息可存储数量
    DiscountStorage: UInt16; // 打折排程可存储数量
    Note1Storage: UInt16; // Note1可存储数量
    Note2Storage: UInt16; // Note2可存储数量
    Note3Storage: UInt16; // Note3可存储数量
    Note4Storage: UInt16; // Note4可存储数量
    Adjunct: array [0 .. 176] of Byte; // 保留参数
  end;

  // 任务句柄
  TASSDKTaskHandle = THandle;

  // 错误信息
  TASSDKError = packed record
    ErrType: Byte;

  end;

  // 在线销售信息
  TASSDKOnlineSellType = (ostSinglePLU, ostDepartmentPLU);

  /// <summary>
  /// <para>
  /// 进度回调函数
  /// </para>
  /// <para>
  /// nErrorCode:错误号
  /// Index:当前行
  /// Total:总行数
  /// lpUserData:用户数据指针
  /// </para>
  /// </summary>
  /// <remarks>
  /// 开始执行：发送一个Index=0,Total=总数 的包处理过程：
  /// 发送一个Index=当前完成行，Total=总数 的包。继续执行
  /// 结束：发送一个Index=总数，Total=总数 的包。继续执行
  /// 异常：ErrorCode=1，Index=异常所在行。退出。
  /// </remarks>
  TASSDKOnProgressEvent = procedure(nErrorCode, Index, Total: Cardinal; lpUserData: Pointer); stdcall;

  /// <summary>
  /// <para>
  /// 在线销售回调函数
  /// </para>
  /// <para>
  /// 在回调过程中需要对AclasSDK_PostOnlineSellData进行调用
  /// </para>
  /// </summary>
  /// <param name="lpUserData">
  /// 用户指针
  /// </param>
  /// <param name="PeerHandle">
  /// 对象句柄
  /// </param>
  /// <param name="TaskType">
  /// 任务类型： 单条PLU或者部门多条PLU
  /// </param>
  /// <param name="PLUNO">
  /// 生鲜码或部门号
  /// </param>
  /// <remarks>
  /// 查询到数据并调用了AclasSDK_PostOnlineSellData后返回True，未查询到数据返回False
  /// </remarks>
  TASSDKOnOnlineSellEvent = function(lpUserData: Pointer; PeerHandle: THandle; TaskType: TASSDKOnlineSellType;
    ID: Integer): Boolean; stdcall;

implementation

end.
