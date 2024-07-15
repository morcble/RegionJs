<div id="REGION" class="hidden"> 
		<div class="form-body" label-position="left" border>
			<div class="col-xs-6 col-md-6 form-item-public is-required">
				<message key="Date" class="form-label"></message>
				<input type="text" region-attr="name" class="ddd form-control" readonly tail="<i class='fa-light fa-calendar-days' onclick='REGION.openDate(event)'></i>">
			</div>	
		</div>		
</div>

<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion  = this;
			var paraMap = new HashMap();
			RS.addDialog(curRegion.find(".ddd"),"common/shared/date/core/time-picker.rs",paraMap,
				function(targetInput,paraMap){//可动态调节组件的参数
					console.log("before dialog show")
					paraMap.put("timeFormat","HH:mm");//例如修改样式
					paraMap.put("initTime",targetInput.val());//例如修改样式
					
					var jqueryDef = new jQuery.Deferred();
					setTimeout(function(){
						jqueryDef.resolve();
					})
					
					return jqueryDef.promise();
				},
				function(dialogRegion,targetInput){//消息框显示的时候
					console.log("after dialog show")
					dialogRegion.onOkClick = function(time){
						console.log(this)
						console.log(targetInput)
						console.log(time)
						targetInput.val(time);
						RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
					}
					dialogRegion.onNowClick = function(time){
						console.log(this)
						console.log(targetInput)
						console.log(time)
						targetInput.val(time);
						RS.tryCloseDialog(targetInput,0);//立即关闭浮动窗口
					}
				});//添加输入悬浮框
			
		},
		openDate:function(event){
			var eventDom = RS.getEventTarget(event);
			$(eventDom).closest(".input-wrapper").find(".wrapped").focus();
		}
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.afterRenderData = REGION.afterRenderData;
	formRegion.renderRegion();
})
</script>
