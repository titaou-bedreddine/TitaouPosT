object frmMain: TfrmMain
  Left = 426
  Top = 223
  Width = 497
  Height = 336
  Caption = 'Demo'
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
    Left = 25
    Top = 35
    Width = 72
    Height = 19
    Caption = 'Device IP:'
  end
  object lblFile: TLabel
    Left = 31
    Top = 73
    Width = 66
    Height = 19
    Caption = 'File Path:'
  end
  object lblDataType: TLabel
    Left = 19
    Top = 104
    Width = 78
    Height = 19
    Caption = 'Data Type:'
  end
  object Label2: TLabel
    Left = 32
    Top = 136
    Width = 334
    Height = 19
    Caption = 'DataType '#35831#21442#32771'ReadMe'#25991#26723#25110#32773'Demo Source'
  end
  object Label3: TLabel
    Left = 32
    Top = 168
    Width = 160
    Height = 19
    Caption = #40664#35748#20540'0000'#26159'PLU'#31867#22411
  end
  object edtIP: TEdit
    Left = 104
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
  object btnDownLoad: TButton
    Left = 32
    Top = 192
    Width = 377
    Height = 33
    Caption = 'DownLoad'
    TabOrder = 1
    OnClick = btnDownLoadClick
  end
  object edtFile: TEdit
    Left = 104
    Top = 70
    Width = 249
    Height = 27
    ImeName = #20013#25991'('#31616#20307') - '#25628#29399#25340#38899#36755#20837#27861
    TabOrder = 2
  end
  object btnFile: TButton
    Left = 359
    Top = 71
    Width = 34
    Height = 25
    Caption = '...'
    TabOrder = 3
    OnClick = btnFileClick
  end
  object ProgressBar: TProgressBar
    Left = 8
    Top = 232
    Width = 456
    Height = 65
    TabOrder = 4
  end
  object edtDataType: TEdit
    Left = 104
    Top = 104
    Width = 249
    Height = 27
    TabOrder = 5
    Text = '0000'
  end
  object OpenDialog: TOpenDialog
    Left = 408
    Top = 32
  end
end
