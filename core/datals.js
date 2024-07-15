
//if(typeof parent.DataList == "undefined"){
	this.DataList = new HashMap();

	this.DataList.put("ErrorType",[{"text":"前端错误","value":"FrontError"},{"text":"后端错误","value":"BackendError"}]);
	this.DataList.put("BeanType",[{"text":"Controller","value":"Controller"},{"text":"Component","value":"Component"},{"text":"Resource","value":"Resource"},{"text":"Service","value":"Service"},{"text":"Dao","value":"Dao"},{"text":"Entity","value":"Entity"}]);
	this.DataList.put("AccountStatus",[{"text":"正常","value":"1"},{"text":"冻结","value":"-1"},{"text":"待激活","value":"0"}]);
	this.DataList.put("EnableDisable",[{"text":"开启","value":"Enabled"},{"text":"关闭","value":"Disabled"}]);
	this.DataList.put("Gender",[{"text":"男","value":"1"},{"text":"女","value":"0"}]);
	//this.DataList.put("SaasSysType",[{"text":"门户网站管理","value":"0"},{"text":"科技园管理","value":"1"}]);
	this.DataList.put("SpStatus",[{"text":"正常","value":"0"},{"text":"冻结","value":"1"}]);

	this.DataList.put("TitlePosition",[{"text":"右对齐","value":"0"},{"text":"左对齐","value":"1"},{"text":"居上","value":"2"}]);
	this.DataList.put("BlockWidth",[{"text":"100%","value":"100%"},{"text":"50%","value":"50%"},{"text":"33.3%","value":"33.3%"},{"text":"25%","value":"25%"}]);
	this.DataList.put("TrueOrFalse",[{"text":"是","value":"true"},{"text":"否","value":"false"}]);
	this.DataList.put("TrueOrFalseStr",[{"text":"是","value":"1"},{"text":"否","value":"0"}]);
	// this.DataList.put("FalseOrTrue",[{"text":"否","value":"1"},{"text":"是","value":"0"}]);

	this.DataList.put("YesOrNo",[{"text":"是","value":"1"},{"text":"否","value":"0"}]);
	this.DataList.put("OpenOrClose",[{"text":"开","value":"1"},{"text":"关","value":"0"}]);
	this.DataList.put("ValidationTypes",[{"text":"无","value":"0"},{"text":"数字","value":"1"},{"text":"手机号码","value":"2"}
		,{"text":"电子邮件","value":"3"},{"text":"IP地址","value":"4"},{"text":"Url","value":"5"}
		,{"text":"正则表达式","value":"6"}]);
	this.DataList.put("SampleOptions",[{"text":"Option1","value":"0"},{"text":"Option2","value":"1"}]);
	this.DataList.put("LayoutType",[{"text":"横向","value":"h"},{"text":"竖向","value":"v"}]);
	this.DataList.put("FileStorageType",[{"text":"本地存储","value":"local"},{"text":"云存储","value":"cloud"}]);
	this.DataList.put("DataSrcType",[{"text":"静态","value":"static"},{"text":"表单属性","value":"formAttr"},{"text":"全局属性","value":"global"}]);


	this.DataList.put("ColumnType",[{"text":"文本","value":"text"},{"text":"单选","value":"s-select"},{"text":"多选","value":"m-select"}]);
	this.DataList.put("ColumnDataSrcType",[{"text":"无","value":""},{"text":"静态","value":"static"},{"text":"全局","value":"global"}]);
	this.DataList.put("DataType",[{"text":"静态","value":"json"},{"text":"数据源","value":"datasource"}]);

	this.DataList.put("DatasourceType",[{"text":"动态数据","value":"1"},{"text":"静态json","value":"0"}]);
	this.DataList.put("DatasourceStatus",[{"text":"未部署","value":"0"},{"text":"已部署","value":"1"}]);
	this.DataList.put("ResType",[{"text":"HTML","value":"0"},{"text":"JSON","value":"1"},{"text":"相对URL地址","value":"2"},{"text":"绝对URL地址","value":"3"}]);

	this.DataList.put("PayMethods",[{"text":"线下支付","value":"1"},{"text":"支付宝","value":"2"}]);

	this.DataList.put("TitlePosition",[{"text":"右对齐","value":"0"},{"text":"左对齐","value":"1"},{"text":"居上","value":"2"}]);
	this.DataList.put("BlockWidth",[{"text":"100%","value":"100%"},{"text":"50%","value":"50%"},{"text":"33.3%","value":"33.3%"},{"text":"25%","value":"25%"}]);
	this.DataList.put("TrueOrFalse",[{"text":"是","value":"true"},{"text":"否","value":"false"}]);
	this.DataList.put("ValidationTypes",[{"text":"无","value":"0"},{"text":"数字","value":"1"},{"text":"手机号码","value":"2"} ,{"text":"电子邮件","value":"3"},{"text":"IP地址","value":"4"},{"text":"Url","value":"5"} ,{"text":"正则表达式","value":"6"}]);
	this.DataList.put("SampleOptions",[{"text":"Option1","value":"0"},{"text":"Option2","value":"1"}]);
	this.DataList.put("LayoutType",[{"text":"横向","value":"h"},{"text":"竖向","value":"v"}]);
	this.DataList.put("FileStorageType",[{"text":"本地存储","value":"local"},{"text":"云存储","value":"cloud"}]);
	this.DataList.put("DataSrcType",[{"text":"静态","value":"static"},{"text":"表单属性","value":"formAttr"},{"text":"全局属性","value":"global"}]);
	this.DataList.put("EnableStatusInt",[{"text":"启用","value":"1"},{"text":"禁用","value":"0"}]);
	this.DataList.put("EnableStatusResult",[{"text":"已启用","value":"1"},{"text":"已禁用","value":"0"}]);

	this.DataList.put("ColumnType",[{"text":"文本","value":"text"},{"text":"单选","value":"s-select"},{"text":"多选","value":"m-select"}]);
	this.DataList.put("ColumnDataSrcType",[{"text":"无","value":""},{"text":"静态","value":"static"},{"text":"全局","value":"global"}]);
	this.DataList.put("DataType",[{"text":"静态","value":"json"},{"text":"数据源","value":"datasource"}]);

	this.DataList.put("DatasourceType",[{"text":"动态数据","value":"1"},{"text":"静态json","value":"0"}]);
	this.DataList.put("DatasourceStatus",[{"text":"未部署","value":"0"},{"text":"已部署","value":"1"}]);
	this.DataList.put("ResType",[{"text":"HTML","value":"0"},{"text":"JSON","value":"1"},{"text":"相对URL地址","value":"2"},{"text":"绝对URL地址","value":"3"}]);
	
	this.DataList.put("RegionType",[{"text":"组件相对URL地址","value":"0"}]);
	this.DataList.put("ApproveMethod",[{"text":"指定角色审核","value":"0"},{"text":"按组织架构审核","value":"1"},{"text":"直接主管审核","value":"2"},{"text":"指定用户审核","value":"3"},{"text":"会签(多人审核)","value":"4"},{"text":"复杂模式(通过调用微服务获取有资格审批的用户列表)","value":"5"}]);
	this.DataList.put("NonGroupApproveMethod",[{"text":"指定角色审核","value":"0"},{"text":"按组织架构审核","value":"1"},{"text":"直接主管审核","value":"2"},{"text":"指定用户审核","value":"3"},{"text":"复杂模式(获取有资格审批的用户列表)","value":"5"}]);

	//workflow start
	this.DataList.put("WorkFlowStatus",[{"text":"草稿","value":"-1"},{"text":"审核中","value":"0"},{"text":"已通过审核","value":"10"},{"text":"未通过审核","value":"11"},{"text":"申请已取消","value":"20"}]);
	this.DataList.put("MsgType",[{"text":"短信","value":"0"},{"text":"电子邮件","value":"1"},{"text":"App推送","value":"2"}]);
	this.DataList.put("MsgContentType",[{"text":"普通文本","value":"0"},{"text":"富文本","value":"1"}]);
	this.DataList.put("StepType",[{"text":"审批者","value":"0"},{"text":"申请人","value":"1"}]);
	this.DataList.put("WFForwardType",[{"text":"流程跳转","value":"0"},{"text":"成功结束","value":"1"},{"text":"失败结束","value":"2"}]);
	//workflow end
	
	this.DataList.put("EnableDisableInt",[{"text":"开启","value":"1"},{"text":"关闭","value":"0"}]);
	this.DataList.put("DisableStatus",[{"text":"禁用","value":"1"},{"text":"启用","value":"0"}]);
	this.DataList.put("EnableStatus",[{"text":"启用","value":"1"},{"text":"禁用","value":"0"}]);
	


