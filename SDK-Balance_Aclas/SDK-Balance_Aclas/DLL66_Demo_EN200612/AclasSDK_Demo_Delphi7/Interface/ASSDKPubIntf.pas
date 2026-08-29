{*******************************************************

  Aclas SDK -- Interface

  Copyright (C) 2014 Aclas. All rights reserved.

  ******************************************************* }
// ------------------------------------------------------------------------------
// 2014-10-09: GLL(6355) Create
// ------------------------------------------------------------------------------
unit ASSDKPubIntf;

interface

uses
  ASSDKPubTypes;

const
{$IFDEF LINUX}
  LibaryName = 'aclassdk';
{$ELSE}
  LibaryName = 'AclasSDK.dll';
{$ENDIF}
  /// <summary>
  /// 初始化动态库
  /// </summary>
  /// <returns>
  /// <para>
  /// 返回True则执行成功
  /// </para>
  /// </returns>
function AclasSDK_Initialize(Adjuct: Pointer = nil): Boolean; stdcall;

/// <summary>
/// 清理和释放动态库
/// </summary>
procedure AclasSDK_Finalize; stdcall;
/// <summary>
/// 获取设备信息
/// </summary>
/// <param name="Addr">
/// 地址
/// </param>
/// <param name="Port">
/// 端口
/// </param>
/// <param name="ProtocolType">
/// 协议类型，未知协议类型请设置为None(0)
/// </param>
/// <returns>
/// 设备信息,当返回值TASSDKDeviceInfo.ProtocolType=0时表示设备获取失败.
/// </returns>
function AclasSDK_GetDeviceInfo(Addr, Port, ProtocolType: Cardinal): TASSDKDeviceInfo; stdcall;

/// <summary>
/// 获取网段机器列表
/// </summary>
/// <param name="Addr">
/// 地址
/// </param>
/// <param name="Port">
/// 端口
/// </param>
/// <param name="ProtocolType">
/// 协议类型，未知协议类型请设置为None(0)
/// </param>
/// <param name="DeviceInfos">
/// 已申请的TASSDKDeviceInfo空间指针。
/// </param>
/// <param name="Count">
/// 申请的空间数量
/// </param>
/// <returns>
/// 返回网段内设备的数量。
/// </returns>
function AclasSDK_GetNetworkSectionDevicesInfo(Addr, Port, ProtocolType: Cardinal; DeviceInfos: Pointer;
  Count: Cardinal): Integer; stdcall;

/// <summary>
/// 操作数据
/// </summary>
/// <param name="Addr">
/// 地址信息
/// </param>
/// <param name="Port">
/// 端口
/// </param>
/// <param name="ProtocolType">
/// 协议类型，未知协议类型请设置为None(0)
/// </param>
/// <param name="ProcType">
/// 操作类型
/// </param>
/// <param name="DataType">
/// 数据类型
/// </param>
/// <param name="FileName">
/// 文件名指针
/// </param>
/// <param name="OnProgress">
/// 操作进度
/// </param>
/// <param name="UserData">
/// 用户数据
/// </param>
/// <returns>
/// <para>
/// 返回设备句柄
/// </para>
/// <para>
/// 当返回值=INVALID_HANDLE_VALUE(-1)表示任务创建失败
/// </para>
/// </returns>
/// <remarks>
/// <para>
/// AclasSDK_ExecTaskA中的FileName为Ansi Char指针
/// </para>
/// <para>
/// AclasSDK_ExecTaskW中的FileName为Wide Char指针
/// </para>
/// <para>
/// AclasSDK_ExecTask = AclasSDK_ExecTaskW
/// </para>
/// </remarks>
function AclasSDK_ExecTaskA(Addr, Port, ProtocolType, ProcType, DataType: Cardinal; FileName: PAnsiChar;
  OnProgress: TASSDKOnProgressEvent; UserData: Pointer): TASSDKTaskHandle; stdcall;
function AclasSDK_ExecTask(Addr, Port, ProtocolType, ProcType, DataType: Cardinal; FileName: PWideChar;
  OnProgress: TASSDKOnProgressEvent; UserData: Pointer): TASSDKTaskHandle; stdcall;
function AclasSDK_ExecTaskW(Addr, Port, ProtocolType, ProcType, DataType: Cardinal; FileName: PWideChar;
  OnProgress: TASSDKOnProgressEvent; UserData: Pointer): TASSDKTaskHandle; stdcall;
/// <summary>
/// 获取最后错误
/// </summary>
/// <returns>
/// 错误值
/// </returns>
function AclasSDK_GetLastTaskError: Integer; stdcall;
/// <summary>
/// 停止任务
/// </summary>
/// <param name="TaskHandle">
/// 任务句柄
/// </param>
/// <remarks>
/// <para>
/// 传入AclasSDK_ExecTask返回的 TASSDKDeviceInfo.TaskHandle停止该任务。
/// </para>
/// <para>
/// 传入TaskHandle=0，则停止所有正在进行的任务。
/// </para>
/// <para>
/// 此函数会确保所要停止的任务完全停止后才返回。
/// </para>
/// </remarks>
procedure AclasSDK_StopTask(TaskHandle: THandle = 0); stdcall;
/// <summary>
/// 等待任务
/// </summary>
/// <param name="TaskHandle">
/// 任务句柄
/// </param>
/// <remarks>
/// <para>
/// 传入AclasSDK_ExecTask返回的 TASSDKDeviceInfo.TaskHandle等待该任务结束。
/// </para>
/// </remarks>
procedure AclasSDK_WaitForTask(TaskHandle: THandle); stdcall;
/// <summary>
/// 在线销售
/// </summary>
/// <param name="lpUserData">
/// 用户指针
/// </param>
/// <param name="Active">
/// 是否启用；True: 启用； False：关闭
/// </param>
/// <param name="OnOnlineSellEvent">
/// 在线销售获取PLU信息事件
/// </param>
/// <param name="ListenPort">
/// 监听端口(隐藏参数)
/// </param>
/// <remarks>
/// <para>
/// 成功返回True，失败返回False
/// </para>
/// </remarks>
function AclasSDK_OnlineSell(lpUserData: Pointer; Active: Boolean; OnOnlineSellEvent: TASSDKOnOnlineSellEvent;
  ListenPort: Cardinal = ASSDK_OnlineSell_DefaultPort): Boolean; stdcall;
/// <summary>
/// 在线销售返回函数
/// </summary>
/// <summary>
/// 当收到OnOnlineSellEvent事件时，调用此参数并带入PeerHandle设置查询后的PLU数据
/// </summary>
/// <param name="PeerHandle">
/// OnOnlineSellEvent返回的PeerHandle
/// </param>
/// <param name="TaskType">
/// 任务类型
/// </param>
/// <param name="lpData">
/// 字符串指针
/// </param>
/// <remarks>
/// <para>
/// 此函数只允许在OnOnlineSellEvent的返回过程中进行调用。
/// </para>
/// </remarks>
function AclasSDK_PostOnlineSellData(PeerHandle: THandle; lpData: PChar): Boolean; stdcall;
function AclasSDK_PostOnlineSellDataA(PeerHandle: THandle; lpData: PAnsiChar): Boolean; stdcall;
function AclasSDK_PostOnlineSellDataW(PeerHandle: THandle; lpData: PChar): Boolean; stdcall;
// function AclasSDK_UserStorage(HostInfo: TASSDKHostInfo; FamilyType: TASSDKFamilyType;
// StorageType: TASSDKUserStorageType; var UserStorage: TASSDKUserStorage): Boolean; stdcall;

implementation

function AclasSDK_Initialize; external LibaryName name 'AclasSDK_Initialize';
procedure AclasSDK_Finalize; external LibaryName name 'AclasSDK_Finalize';
function AclasSDK_GetDeviceInfo; external LibaryName name 'AclasSDK_GetDeviceInfo';
function AclasSDK_GetNetworkSectionDevicesInfo; external LibaryName name 'AclasSDK_GetNetworkSectionDevicesInfo';
function AclasSDK_ExecTaskA; external LibaryName name 'AclasSDK_ExecTaskA';
function AclasSDK_ExecTask; external LibaryName name 'AclasSDK_ExecTask';
function AclasSDK_ExecTaskW; external LibaryName name 'AclasSDK_ExecTaskW';
function AclasSDK_GetLastTaskError; external LibaryName name 'AclasSDK_GetLastTaskError';
procedure AclasSDK_StopTask; external LibaryName name 'AclasSDK_StopTask';
procedure AclasSDK_WaitForTask; external LibaryName name 'AclasSDK_WaitForTask';
function AclasSDK_OnlineSell; external LibaryName name 'AclasSDK_OnlineSell';
function AclasSDK_PostOnlineSellData; external LibaryName name 'AclasSDK_PostOnlineSellData';
function AclasSDK_PostOnlineSellDataA; external LibaryName name 'AclasSDK_PostOnlineSellDataA';
function AclasSDK_PostOnlineSellDataW; external LibaryName name 'AclasSDK_PostOnlineSellDataW';
// function AclasSDK_UserStorage; external LibaryName name 'AclasSDK_UserStorage';

end.
