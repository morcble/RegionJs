<style type="text/css">
#REGION{
   background: yellow;
}

#REGION input{
	margin-right: 20px;
}

</style>

<div id="REGION" class="hidden">
	<input id="test" type="button" onclick="REGION.gotoScreen(event)" value="跳转到A"/>
	<input type="button" onclick="history.go(-1)" value="浏览器回退"/>
	<input type="button" onclick="REGION.screenGoBack(event)" value="Screen回退"/>
</div>	


<script type="text/javascript">
var REGION = {
		screenGoBack:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			var screenHolderRegion = curRegion.screenHolderRegion;
			screenHolderRegion.goBack();
		},
		gotoScreen:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			console.log(curRegion.screenHolderRegion)
			var paraMap = new HashMap();
			paraMap.put("bbb","2")
			RegionUtil.gotoScreen(curRegion,"xscreen",paraMap);
		},
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			console.log("render region b")
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
