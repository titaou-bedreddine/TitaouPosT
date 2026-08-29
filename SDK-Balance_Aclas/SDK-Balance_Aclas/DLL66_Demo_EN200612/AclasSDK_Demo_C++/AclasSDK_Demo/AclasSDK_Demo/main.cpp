// main.cpp
//

#include "stdafx.h"
#include <windows.h>
#pragma comment(lib,"ws2_32.lib")
#include <iostream>
using namespace std;
struct DviIn
{
	UINT32 ProtocolType;
	UINT32  Addr;
	UINT32  Port;
	UCHAR    name[16];
	UINT32  ID;
	UINT32  Version;
	BYTE     Country;
	BYTE     DepartmentID;
	BYTE     KeyType;
	UINT64  PrinterDot;
	LONG64   PrnStartDate;
	UINT32  LabelPage;
	UINT32  PrinterNo;
	USHORT   PLUStorage;
	USHORT   HotKeyCount;
	USHORT   NutritionStorage;
	USHORT   DiscountStorage;
	USHORT   Note1Storage;
	USHORT   Note2Storage;
	USHORT   Note3Storage;
	USHORT   Note4Storage;
	BYTE     stroge[177];
};

extern "C" 
{
	typedef bool (CALLBACK*pAclasSDKInitialize)( char *s);
	typedef bool (CALLBACK*pGetDevicesInfo)(UINT32  Addr,UINT32  Port,UINT32 ProtocolType ,DviIn *info);

    typedef void (WINAPI *FP)(UINT32 Eorrorcode,UINT32 index,UINT32 Total,char *userdata);

	typedef HANDLE (CALLBACK*pAclasSDKExecTask)(UINT32 Addr,UINT32  Port,UINT32 ProtocolType,UINT32 ProceType,UINT32 DataType,char *FileName,FP fp ,char *uerdata);
	typedef HANDLE (CALLBACK*pAclasSDKWaitForTask)(HANDLE handle);

	typedef int (CALLBACK*pAclasSDKSyncExecTask)(char *Addr,UINT32  Port,UINT32 ProtocolType,UINT32 ProceType,UINT32 DataType,char *FileName);
}	

void WINAPI ongress(UINT32 Eorrorcode,UINT32 index,UINT32 Total,char *userdata)
{
	
   switch (Eorrorcode)
    {
     case 0x0000:
		 cout<<"complete"<<endl;
	   break;
	 case 0x0001:
	     cout<<index<<"/"<<Total<<endl;
	   break;
     }
}
UINT MakehostToDword(char *host)
{
	UINT result;
	UINT a[4];
	char *p1=NULL;

	char str[20];
	strcpy(str,host);
	p1=strtok(str,".");
	a[0]=atoi(p1);
	result=a[0]<<24;
	for (int i=1;i<4;i++)
	{
		p1=strtok(NULL,".");
		a[i]=atoi(p1);
		result+=a[i]<<((3-i)*8);
	}  
	return result;


}

int _tmain(int argc, _TCHAR* argv[])
{
	BOOL sta=false;
	BOOL ref=false;
	HMODULE hModule = LoadLibrary(L"AclasSDK.dll"); 
	if (hModule)
	{
		// Initialize
		pAclasSDKInitialize Initialize=(pAclasSDKInitialize)GetProcAddress(hModule, "AclasSDK_Initialize");
		char *str=NULL;
		sta=Initialize(str);
		if(sta)
		{
			cout<<"初始化成功"<<endl;
		}
		else
		{
			cout<<"初始化失败"<<endl;
		}
		//Get Device Information
        pGetDevicesInfo getDevicesInfo=(pGetDevicesInfo)GetProcAddress(hModule, "AclasSDK_GetDevicesInfo");
		struct DviIn *info;
		 info = (struct DviIn *)malloc(sizeof(struct DviIn));
		char *Host=("192.168.2.234"); //change to your scale ip
		 UINT addr=MakehostToDword(Host);

		ref=getDevicesInfo(addr,0,0,info);
		cout<<info->name;
		FP fp;
		fp=ongress;
		
		HANDLE handle;
		char *userdata=NULL;
		char *path="d:\\ex\\PLU Demo.txt"; //change to your plu file

		//ASync call
		
		pAclasSDKExecTask exectask=(pAclasSDKExecTask)GetProcAddress(hModule, "AclasSDK_ExecTaskA");
		pAclasSDKWaitForTask waitfortask=(pAclasSDKWaitForTask)GetProcAddress(hModule, "AclasSDK_WaitForTask");
		handle=waitfortask(exectask(addr,5002,info->ProtocolType,0,0x0000,path, fp,userdata));
		

		//Sync call
		/*
		pAclasSDKSyncExecTask syncexectask=(pAclasSDKSyncExecTask)GetProcAddress(hModule, "AclasSDK_Sync_ExecTaskA_PB");
		int retval=syncexectask(Host,5002,info->ProtocolType,0,0x0000,path);
		cout<<retval<<endl;
		*/
	}
	return 0;
}

