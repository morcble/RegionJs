<style type="text/css">


</style>
<!-- 
1. 最大行数限制  默认为20
2. 类型分为 尾部追加和头部追加。添加数据时，追加到列表末尾或者头部
3. 当窗口需要滚动时，自动往上滚动或者往下滚动
4. 数据增加有动画效果 
-->
<div id="REGION" class="hidden">
	<div class="list-holder"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			if(!curRegion.inited){
				curRegion.inited = true;
		
				var paraMap = new HashMap();
				//paraMap.put("append","top");
				//paraMap.put("maxRowNum","100");
				paraMap.put("autoScroll","true");
				paraMap.put("rowTemplate",`<div class="row">
						<div class="col-xs-2"><span region-attr="time"></span></div>
						<div class="col-xs-1"><span region-attr="type"></span></div>
						<div class="col-xs-9"><span region-attr="content"></span></div>
						</div>`);
				var loadListPromise = RS.loadRegion(curRegion.find(".list-holder"),RS.appendParaMapToUrl("common/shared/list/dynamicList.rs",paraMap));
				loadListPromise.done(function(loadedRegion){
					curRegion.dynamicListRegion = loadedRegion;
					
					curRegion.dynamicListRegion.addRow({"time":"2023-09-06 15:08:42","biz":1,"type":1,"content":"start check cluster: 4715299828110721024 rs轻量集群1"});
				}
			}
			
		}
};


RegionUtil.ready(function(){
	var region = RegionUtil.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.renderRegion();
})
</script>


