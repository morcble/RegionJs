<style type="text/css">


</style>

<div id="REGION" class="hidden">
	<div class="holder"></div>
	
	<button class="btn"  data-type="primary" normal onclick="REGION.getListData(event)">获取数据</button>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var listData = [];
			var rowData = {};
			rowData.name = "feng1";
			rowData.status = 1;
			listData.push(rowData);
			
			rowData = {};
			rowData.name = "feng2";
			rowData.status = 0;
			listData.push(rowData);
			
			
			
			var testRegion = this;
			var paraMap = new HashMap();
			paraMap.put("rowRegion","common/shared/list/rowRegion.rs");
			paraMap.put("listData",listData);
			var promise = RegionUtil.loadRegion(testRegion.find(".holder"),RegionUtil.appendParaMapToUrl("common/shared/list/editablelist.rs",paraMap));
			promise.done(function(loadedRegion){
				testRegion.editableListRegion = loadedRegion;
			});
		},
		getListData:function(event){
			var testRegion = RegionUtil.getRegionByEvent(event);
			var result= testRegion.editableListRegion.getListData();
			console.log(result)
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>