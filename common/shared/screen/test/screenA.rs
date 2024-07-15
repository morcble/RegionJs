<style type="text/css">
#REGION{
   background: blue;
}

</style>

<div id="REGION" class="hidden">
	<input type="button" onclick="REGION.gotoScreen(event)" value="跳转到B"/>
</div>	


<script type="text/javascript">
var REGION = {
		gotoScreen:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			var paraMap = new HashMap();
			paraMap.put("aaa","31")
			console.log(curRegion.screenHolderRegion)
			RegionUtil.gotoScreen(curRegion,"yscreen",paraMap);
		},
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			console.log("render region a")
			console.log("aaa="+this.paraMap.get("aaa"))
			console.log("bbb="+this.paraMap.get("bbb"))
			console.log("ccc="+this.paraMap.get("ccc"))
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>
