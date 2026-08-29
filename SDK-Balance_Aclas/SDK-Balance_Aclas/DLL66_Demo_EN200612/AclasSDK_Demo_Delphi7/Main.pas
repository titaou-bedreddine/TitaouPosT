unit Main;

interface

uses
  Windows, Messages, SysUtils, Variants, Classes, Graphics,
  Controls, Forms, Dialogs, StdCtrls, ComCtrls,
  //
  ASSDKPubTypes, ASSDKPubIntf;

const
  m_User = WM_USER + 5;

type
  TfrmMain = class(TForm)
    edtIP: TEdit;
    lblIP: TLabel;
    btnDownLoad: TButton;
    lblFile: TLabel;
    edtFile: TEdit;
    btnFile: TButton;
    OpenDialog: TOpenDialog;
    ProgressBar: TProgressBar;
    lblDataType: TLabel;
    Label2: TLabel;
    Label3: TLabel;
    edtDataType: TEdit;
    procedure btnFileClick(Sender: TObject);
    procedure btnDownLoadClick(Sender: TObject);
    procedure OnDllEvent(var msg: TMessage); message m_User;
    procedure FormClose(Sender: TObject; var Action: TCloseAction);
  private
    { Private declarations }
  public
    { Public declarations }
  end;

var
  frmMain: TfrmMain;

function MakeHostToDWord(sHost: string): Cardinal;

implementation

{$R *.dfm}

procedure OnProgress(nErrorCode, AIndex, ATotal: Cardinal;
lpUserData: Pointer); stdcall;
begin
  case nErrorCode of
    ASSDK_Err_Success: // Complete
      begin
        ShowMessage('OK');
      end;
    ASSDK_Err_Progress: // Progress
      begin
        PostMessage(frmMain.Handle, m_User, AIndex, ATotal);
      end;
  else
    begin
      ShowMessage('false');// false
    end;
  end;
end;

procedure TfrmMain.btnDownLoadClick(Sender: TObject);
var
  ASSDKDeviceInfo: TASSDKDeviceInfo; // DeviceInfo
  iAddr: Integer;
  DataType: Integer;
begin
  // 初始化
  AclasSDK_Initialize;

  DataType := StrToIntDef(edtDataType.Text, -1);
  if DataType = -1 then
  begin
    ShowMessage('DataType Wrong!');
    exit;
  end;

  iAddr := MakeHostToDWord(edtIP.Text);
  ASSDKDeviceInfo := AclasSDK_GetDeviceInfo( //
    iAddr, //
    0, // 默认为 0
    ASSDK_ProtocolType_None // 默认为 ASSDK_ProtocolType_None
    );

  if ASSDKDeviceInfo.ProtocolType = ASSDK_ProtocolType_None then
    exit;  // 未找到设备

  AclasSDK_ExecTaskA( //
    ASSDKDeviceInfo.Addr, // 地址
    ASSDKDeviceInfo.Port, // 端口
    ASSDKDeviceInfo.ProtocolType, // 协议类型
    ASSDK_ProcType_Down, // 操作类型
    DataType, // 数据类型
    PAnsiChar(edtFile.Text), // 下载的文件名
    @OnProgress,
    nil);

  // 清除PLU写法
  {AclasSDK_ExecTaskA( //
    ASSDKDeviceInfo.Addr, // 地址
    ASSDKDeviceInfo.Port, // 端口
    ASSDKDeviceInfo.ProtocolType, // 协议类型
    ASSDK_ProcType_Del, // 操作类型
    ASSDK_DataType_PLU, // 数据类型
    PAnsiChar(string('*')), // 下载的文件名
    @OnProgress,
    nil);}
end;

procedure TfrmMain.btnFileClick(Sender: TObject);
begin
  if OpenDialog.Execute then
    edtFile.Text := OpenDialog.FileName;
end;

procedure TfrmMain.FormClose(Sender: TObject; var Action: TCloseAction);
begin
  AclasSDK_Initialize;
  AclasSDK_Finalize;
end;

procedure TfrmMain.OnDllEvent(var msg: TMessage);
begin
  ProgressBar.Max := msg.LParam;
  ProgressBar.Position := msg.WParam;
end;

function SplitString(sSource: string; Separator: Char): Variant;
var
  iIndex,  iCount: Integer;
  sTemp: string;
  ArrTemp: Array of string;
begin
  iCount := 0;
  sTemp := sSource;
  iIndex := Pos(Separator, sTemp);
  //setLength(OutStr,iCount+1);
  //for iINdex := 0 to Length(sSource) do \
  while (iIndex > 0) and (sTemp <> '') do
  begin
    setLength(ArrTemp, iCount+1);
    ArrTemp[iCount] := Copy(sTemp, 0, iIndex-1);
    iCount := iCount + 1;
    sTemp := Copy(sTemp, iIndex+1, length(sTemp));
    iIndex := Pos(Separator, sTemp);
  end;
  if sTemp <> '' then
  begin
    setLength(ArrTemp, iCount+1);
    ArrTemp[iCount] := sTemp;
    iCount := iCount + 1;
  end;
//  Result := VarArrayCreate([0, iCount-1], varVariant);
  Result := ArrTemp;
  //setLength(OutStr,iCount+1);
end;

function MakeHostToDWord(sHost: string): Cardinal;
var
  i: Integer;
  Segment: array of string;
begin
  Result := 0;
  Segment := SplitString(sHost, '.');
  //Segment := sHost.Split(['.']);
  if Length(Segment) <> 4 then
    Exit;
  for i := 0 to Length(Segment) - 1 do
  begin
    if StrToIntDef(Segment[i], -1) in [0 .. 255] then
      Result := Result + Cardinal(StrToInt(Segment[i]) shl ((3 - i) * 8))
    else
      break;
  end;
end;


end.
