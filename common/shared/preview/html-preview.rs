<style type="text/css">
#REGION>.innerPart{
	height:100%;
	width:100%;
	border: 0;
	overflow: hidden;
	display: block;
	/*border: 1px solid var(--theme-f2);*/
}


</style>

<div id="REGION" class="hidden">
	<iframe class="innerPart"></iframe>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){

			var curRegion = this;
			var url = curRegion.paraMap.get("url");
			var target = curRegion.paraMap.get("target");
			if(target!=null){
				window.open(url,target);
			}
			else{
				
				var marginTop = curRegion.paraMap.get("mt");
				if(marginTop!=null)curRegion.find(".innerPart").css("margin-top",marginTop);
				
				curRegion.find(".innerPart").attr("src",url);
			}
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>