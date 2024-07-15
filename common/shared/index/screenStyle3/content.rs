<style type="text/css">
#REGION{
	
	/* height: auto !important; */
	height: 100%;
}

#REGION>.content-holder>.content>.tab{
	width: 100%;
	height: 100%;
	padding-top: 0px !important;
}

#REGION>.content-holder{
	padding:10px;
	overflow: auto;
	height: 100%;
	box-sizing: border-box;
}

</style>

<div id="REGION" class="hidden">
	<div class="content-holder background-f3">
		<div class="skin content" style="height: 100%;overflow:auto;background: white;position: relative">
		</div>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		loadTab:function(menuId,regionUrl,label,useCache){
			var curRegion = this;
			RegionUtil.loadRegion(curRegion.find(".content"),regionUrl);
		},
		renderContent:function(){
			var curRegion = this;
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.loadTab = REGION.loadTab;
	staticRegion.renderRegion();
})
</script>

