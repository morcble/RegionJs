<div id="REGION" class="hidden"> 
	<div class="form-body"  border>
			<div class="col-xs-12 col-md-12 form-item-public is-required">
				<message key="oldpassword" class="form-label"></message>
				<input region-attr="oldpwd" type="password" class="form-control" maxlength="40">
			</div>
			<div class="col-xs-12 col-md-12 form-item-public is-required">
				<message key="password" class="form-label"></message>
				<input region-attr="pwd" type="password" class="form-control" maxlength="40">
			</div>
			<div class="col-xs-12 col-md-12 text-center footer">
				<button class="btn " data-type="primary" normal onclick="REGION.saveRegion(event)"><message key="global_msg.save"></message></button>
				<button class="btn " data-type="default" normal onclick="REGION.closeModalWindow()"><message key="global_msg.reset"></message></button>
			</div>
	</div>		
</div>
 


<script type="text/javascript">
var REGION = { 
	saveRegion:function(event){
		var tmpRegion = RegionUtil.getRegionByEvent(event);
		var valid = tmpRegion.validate();
		if(valid){
			RegionUtil.confirm("确认修改密码吗?","提示",function(){
				 tmpRegion.saveOrUpdateRegion();
			});
		}
		
	},
	refreshRegion:function(event){
		var tmpRegion = RegionUtil.getRegionByEvent(event);
		tmpRegion.refreshRegion();
	},
	saveSuccessCallBack:function(){
		try{
			var alertDef = RegionUtil.alert("密码修改成功");
			var region = this;
			alertDef.done(function(){
				RegionUtil.closeModalWindow();
				LoginPlugin.logout();
			})
		}catch(e){
			console.log(e)
		}
	},
	saveFailedCallBack:function(){
		console.log("save failed");
	},
	reset:function(event){
		var tmpRegion = RegionUtil.getRegionByEvent(event);
		if(confirm(getMsg("global_msg.reset_confirm"))){
			tmpRegion.reset();
		}
		
	},
}

RegionUtil.ready(function(){
	var res = {
			message:{
				dft:{
					oldpassword:"输入旧密码",
					password:"输入新密码",
					save:"确认",
					reset:"取消",
				}
			},
			dataList:{
				dft:{
					//sample:[{"text":"label1","value":"val1"},{"text":"label12","value":"val2"}]
				}
			}
	};
		var formRegion = RegionUtil.newFormRegion("#REGION",res);
		formRegion.addValidator("oldpwd",new Array(emptyReg),new Array(global_msg.mandatory_fields));
		formRegion.addValidator("pwd",new Array(emptyReg),new Array(global_msg.mandatory_fields));
		formRegion.dataIdPara = "recordId";
		//formRegion.saveUrl = Config.backendPath+"/RCS_Auth/user/save";
		formRegion.saveUrl = Config.backendPath+"/RCS_Auth/user/updatePwd";
		formRegion.saveSuccessCallBack = REGION.saveSuccessCallBack;
		formRegion.saveFailedCallBack = REGION.saveFailedCallBack;
			
		formRegion.beforeRetrieveData = REGION.beforeRetrieveData;
		formRegion.afterRetrieveData = null;
		formRegion.beforeRenderData = null;
		formRegion.afterRenderData = REGION.afterRenderData;
		formRegion.beforeValidate = REGION.beforeValidate;
		formRegion.afterValidate = REGION.afterValidate;
		formRegion.renderRegion();
	}); 
</script>




