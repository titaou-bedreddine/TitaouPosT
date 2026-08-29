object frmMain: TfrmMain
  Left = 0
  Top = 0
  Caption = 'Demo'
  ClientHeight = 475
  ClientWidth = 472
  Color = clBtnFace
  Font.Charset = DEFAULT_CHARSET
  Font.Color = clWindowText
  Font.Height = -16
  Font.Name = 'Tahoma'
  Font.Style = []
  OldCreateOrder = False
  Position = poScreenCenter
  OnClose = FormClose
  PixelsPerInch = 96
  TextHeight = 19
  object lblIP: TLabel
    Left = 33
    Top = 35
    Width = 72
    Height = 19
    Caption = 'Device IP:'
  end
  object lblFile: TLabel
    Left = 39
    Top = 73
    Width = 66
    Height = 19
    Caption = 'File Path:'
  end
  object lblDataType: TLabel
    Left = 27
    Top = 110
    Width = 78
    Height = 19
    Caption = 'Data Type:'
  end
  object lblReadMe: TLabel
    Left = 8
    Top = 148
    Width = 334
    Height = 19
    Caption = 'DataType '#35831#21442#32771'ReadMe'#25991#26723#25110#32773'Demo Source'
  end
  object lblReadMe2: TLabel
    Left = 8
    Top = 176
    Width = 160
    Height = 19
    Caption = #40664#35748#20540'0000'#26159'PLU'#31867#22411
  end
  object edtIP: TEdit
    Left = 117
    Top = 32
    Width = 201
    Height = 27
    Font.Charset = DEFAULT_CHARSET
    Font.Color = clWindowText
    Font.Height = -16
    Font.Name = 'Tahoma'
    Font.Style = []
    ImeName = #20013#25991'('#31616#20307') - '#25628#29399#25340#38899#36755#20837#27861
    ParentFont = False
    TabOrder = 0
  end
  object btnDownLoadPLU: TButton
    Left = 8
    Top = 206
    Width = 456
    Height = 33
    Caption = 'DownLoad'
    TabOrder = 1
    OnClick = btnDownLoadPLUClick
  end
  object edtFile: TEdit
    Left = 117
    Top = 70
    Width = 249
    Height = 27
    ImeName = #20013#25991'('#31616#20307') - '#25628#29399#25340#38899#36755#20837#27861
    TabOrder = 2
  end
  object btnFile: TButton
    Left = 372
    Top = 71
    Width = 34
    Height = 25
    Caption = '...'
    TabOrder = 3
    OnClick = btnFileClick
  end
  object ProgressBar: TProgressBar
    Left = 8
    Top = 285
    Width = 456
    Height = 65
    TabOrder = 4
  end
  object edtDatatype: TEdit
    Left = 116
    Top = 107
    Width = 250
    Height = 27
    ImeName = #35895#27468#25340#38899#36755#20837#27861' 2'
    TabOrder = 5
    Text = '0000'
  end
  object Memo: TMemo
    Left = 0
    Top = 361
    Width = 472
    Height = 114
    Align = alBottom
    ImeName = #35895#27468#25340#38899#36755#20837#27861' 2'
    TabOrder = 6
    ExplicitTop = 336
  end
  object btnSyncDownLoadPLU: TButton
    Left = 8
    Top = 246
    Width = 456
    Height = 33
    Caption = 'Sync DownLoad'
    TabOrder = 7
    OnClick = btnSyncDownLoadPLUClick
  end
  object OpenDialog: TOpenDialog
    Left = 408
    Top = 32
  end
end
