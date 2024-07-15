<div id="REGION" class="hidden"> 
		<div class="form-body" label-position="left" border>
			<div class="col-xs-6 col-md-6 form-item-public is-required">
				<message key="Date" class="form-label"></message>
				<input type="text" region-attr="name" class="ddd form-control" readonly tail="<i class='fa-light fa-calendar-days' onclick='REGION.openDate(event)'></i>">
			</div>	

			<div class="col-xs-12 col-md-12 form-item-public is-required">
				
			</div>	
		</div>		
</div>

<script type="text/javascript">
var REGION = {
		
		afterRenderData:function(){
			var curRegion  = this;
			
			var paraMap = new HashMap();
			RS.addDialog(curRegion.find(".ddd"),"common/shared/date/core/rangedate-picker.rs",paraMap,
					function(targetInput,paraMap){//可动态调节组件的参数
						var timeRangeVal = targetInput.val();
						if(timeRangeVal!=null){
							if(timeRangeVal.indexOf("~")!=-1){
								timeRangeVal = timeRangeVal.replaceAll(" ","");
								var timeRangeArray = timeRangeVal.split("~");
								if(timeRangeArray.length==2){
									paraMap.put("startDate",timeRangeArray[0]);
									paraMap.put("endDate",timeRangeArray[1]);
								}
							}
						}
						console.log("before shown")
					},
					function(dialogRegion,targetInput){//消息框显示的时候
						dialogRegion.onRangeSelectEnd = function(date){
							console.log(this)
							console.log(targetInput)
							console.log(date)
							targetInput.val(date.startDate+" ~ "+date.endDate);
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
