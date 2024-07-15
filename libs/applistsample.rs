<div id="REGION" class="hidden">
<style type="text/css">
.cellcontent { border-left:1px solid #000}


/* .mui-table-view > div> .mui-table-view-cell:nth-child(odd) {
	background: white;
}
.mui-table-view > div > .mui-table-view-cell:nth-child(even) {
	background: #efeff4;
} */

/* global style */
body {
    font-family: Frutiger, Arial, sans-serif !important;
    font-size: 16px;
    color: black;
    background-color: white;
   
}

*{
	user-select:text !important; /* for web only *//* 如果是app 则改为none */
}


.hidden {
    display: none!important;
}

.hide {
    display: none!important;
}



.form-header{
	font-weight: bold;
    border-color: #bce8f1;
    padding: 10px 15px;
    border-bottom: none;
    border-radius: 0;
    background:#efeff4;
}

.white-background{
	background:#fff;
}

.padding-bottom-5px{
	padding-bottom: 5px;
}

.padding-top-5px{
	padding-top: 5px;
}

select {
    border: 1px solid rgba(0,0,0,.2)!important; /* for web only */
    -webkit-appearance: menulist!important;
}

/* list style */

.list-header{
	color: #999999;
    line-height: 1.2em;
    font-size: 14px;
}

.region-list>.mui-table-view-cell:hover{background:#effff4;}

.mui-row li{overflow: hidden;text-overflow: ellipsis;padding-right:10px; white-space: nowrap; } 

.mui-row a{color:#337ab7}

.mui-content{
	background-color: white !important;
}



</style>
	<div class="mui-content">
		<div class="form-header">
	<!-- 			<i class="fa fa-fw fa-minus fa-lg collapsePanel"></i> -->
				<span>用户列表</span>
		</div>
		
		<div class="white-background padding-bottom-5px padding-top-5px">
				<button type="button" class="mui-btn mui-btn-outlined regionSearchPrompt">
					<message key="global_msg.search"></message>
				</button>
				<button type="button" class="mui-btn mui-btn-outlined regionNewBtn">
					<message key="global_msg.add"></message>
					<!-- <input type="hidden" class="parameter"  paraName="paraName" region-attr="valType"> -->
				</button>
		</div>
		
		<div class="region-search-div hidden">
					<div class="row margin-bottom-15px">
						<div class="col-xs-6 flex-center">
							<div class="col-xs-4">
								<message key="user_msg.account"></message>
							</div>
							<div class="col-xs-8">
								<input type="text" region-attr="account" class="region-searchable form-control"  maxlength="20">
							</div>
						</div>
					</div>
					<div class="row margin-bottom-15px">
						<div class="col-xs-6 flex-center">
							<div class="col-xs-4">
								<message key="user_msg.status"></message>
							</div>
							<div class="col-xs-8">
								<select class="region-searchable form-control" region-attr="status" region-ds="AccountStatus"><option value="">请选择</option>
								</select>
							</div>
						</div>
					</div>
		
		
				<div class="mui-row">
							<button class="mui-btn mui-btn-outlined regionSearchBtn"><message key="global_msg.submit"></message></button>&nbsp;
							<button class="mui-btn mui-btn-outlined regionResetBtn"><message key="global_msg.reset"></message></button>
				</div>
		</div>
	
	
		<div class="mui-table-view">
				<div class="mui-table-view-cell list-header">
			    	<div class="mui-row">
				    	<li class="mui-col-sm-1 mui-col-xs-12"><message key="global_msg.index"></message></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><message key="user_msg.account"></message></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><message key="user_msg.status"></message></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><message key="user_msg.description"></message></li>
				    	<li class="mui-col-sm-2 mui-col-xs-12" region-order="createDt"><message key="global_msg.create_dt"></message></li>
				    	
				    	<li class="mui-col-sm-1 mui-col-xs-12"><message key="global_msg.create_by"></message></li>
				    	<li class="mui-col-sm-2 mui-col-xs-12" region-order="updateDt"><message key="global_msg.update_dt"></message></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><message key="global_msg.update_by"></message></li>
				    	<li class="mui-col-sm-2 mui-col-xs-12"><message key="global_msg.operation"></message></li>
			    	</div>
			    </div>
	
		
			<div region-list="list">
				<div class="mui-table-view-cell">
			    	<div class="mui-row">
				    	<li class="mui-col-sm-1 mui-col-xs-12"><span region-attr="index"></span></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><span region-attr="account"></span></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><span region-attr="status" region-ds="AccountStatus" ></span></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><span region-attr="description"></span></li>
				    	<li class="mui-col-sm-2 mui-col-xs-12"><span region-attr="createDt"></span></li>
				    	
				    	<li class="mui-col-sm-1 mui-col-xs-12"><span region-attr="createBy"></span></li>
				    	<li class="mui-col-sm-2 mui-col-xs-12"><span region-attr="updateDt"></span></li>
				    	<li class="mui-col-sm-1 mui-col-xs-12"><span region-attr="updateBy"></span></li>
				    	<li class="mui-col-sm-2 mui-col-xs-12">
								<a href="javascript:void(0)" onclick="RegionUtil.handleListData(REGION.changePwd,event)"><i class="fa fa-key fa-fw" title="更改密码"></i></a>
								&nbsp;
								<a region-hide="rowData['status']==1" href="javascript:void(0)" onclick="RegionUtil.handleListData(REGION.lockUser,event)"><i class="fa fa-lock fa-fw" title="冻结用户"></i>&nbsp;</a>

								<a region-hide="rowData['status']==0" href="javascript:void(0)" onclick="RegionUtil.handleListData(REGION.unlockUser,event)"><i class="fa fa-unlock fa-fw" title="解冻用户"></i>&nbsp;</a>
								
								<a href="javascript:void(0)"  onclick="RegionUtil.handleListData(REGION.deleteData)"><i class="fa-solid fa-trash-alt fa-fw" title="删除用户"></i></a>
						</li>
			    	</div>
			    </div>
			</div>
			
			<div class="paginationControl"></div>
								
			<div class="norecordmsg fa hidden"><message key="global_msg.no_record_found"></message></div>

		</div>
	</div>

</div>	

<script type="text/javascript">
var REGION = {
		changePwd :function (rowData){
			this.viewRecord(rowData,this.regionId,null,"admin/user/userview.rs");
		},
		lockUser :function (rowData,evt){
			var region = this;
			RegionUtil.loadingStart();
			var reqObj = new Object();
			reqObj.id = rowData.id;
			reqObj.version = rowData.version;
			
			RegionUtil.ajaxJsonTask(Config.backendPath+"/admin/user/lockuser","POST",reqObj,function(data,dataPara){
				RegionUtil.alert("账户已冻结");
				RegionUtil.loadingComplete();
				
				var index = rowData.index;
				var newRegionData = RegionUtil.clone(region.regionData);
				console.log(newRegionData["list"])
				console.log(index)
				newRegionData["list"][index].status = 1;
				newRegionData["list"][index].version++;
				region.renderRegionWithData(newRegionData);
				//region.refreshRegion();
			})
		},
		unlockUser :function (rowData){
			var region = this;
			RegionUtil.loadingStart();
			var reqObj = new Object();
			reqObj.id = rowData.id;
			reqObj.version = rowData.version;
		
			RegionUtil.ajaxJsonTask(Config.backendPath+"/admin/user/unlockuser","POST",reqObj,function(data,dataPara){
				RegionUtil.alert("账户解冻");
				RegionUtil.loadingComplete();
				
				region.refreshRegion();
			})
		},
		deleteData:function(rowData){
			this.deleteData(rowData);
		},
		/*
		it will block form submission if return false
		*/
		beforeValidate:function(){
			return true;
		},
		afterValidate:function(){
		}

}


RegionUtil.ready(function(){
	var task = RegionUtil.loadMessage("messages_"+Config.locale+"/user_msg.js");
	task.done(function(){
		var regionGrid = new RegionGrid("#REGION");
		//regionGrid.addValidator("account",new Array(emptyReg),new Array(global_msg.mandatory_fields));
		//regionGrid.addValidator("pwd",new Array(emptyReg),new Array(global_msg.mandatory_fields));
		//regionGrid.addValidator("status",new Array(emptyReg),new Array(global_msg.mandatory_fields));
		regionGrid.resUrl = Config.backendPath+  "/admin/user/list";
			
		regionGrid.addTitle = "新增用户";//view title
		regionGrid.viewTitle = "编辑用户";//view title
		regionGrid.popupSize = "m";
		regionGrid.reqRes = "/admin/user/userview.rs";
		regionGrid.beforeRetrieveData = null;
		regionGrid.afterRetrieveData = null;
		regionGrid.beforeRenderData = null;
		regionGrid.afterRenderData = null;
		regionGrid.beforeValidate = REGION.beforeValidate;
		regionGrid.afterValidate = REGION.afterValidate;
		regionGrid.deleteCallBack = null;
		regionGrid.search();		
	});
	
	
});

</script>



