<style>
#REGION{
	display: flex;
}

#REGION>div{
	width: 300px;
	margin: 50px;
}

</style>
<div id="REGION" class="hidden">
	<div class="screenHolder match-parent"></div>
	<div class="screenHolder2 match-parent"></div>
</div>	


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			var curRegion = this;
			var def = RegionUtil.loadRegion(curRegion.find(".screenHolder"),RegionUtil.appendParaMapToUrl("common/shared/screen/screen-new.rs",curRegion.paraMap),"screen1");
			def.done(function(loadedRegion){
				var screensInfo = [
					{'name':'xscreen','screenRegion':'common/shared/screen/test/screenA.rs','useCache':true },
					
					{'name':'yscreen','screenRegion':'common/shared/screen/test/screenB.rs','useCache':true },

				];
				loadedRegion.render(screensInfo);
				//loadedRegion.setAnimation(0);
			});
			
		/* 	var def = RegionUtil.loadRegion(curRegion.find(".screenHolder2"),RegionUtil.appendParaMapToUrl("common/shared/screen/screen-new.rs",curRegion.paraMap),"screen2");
			def.done(function(loadedRegion){
				var screensInfo = [
					{'name':'xscreen','screenRegion':'common/shared/screen/test/screenA.rs','useCache':true },
					
					{'name':'yscreen','screenRegion':'common/shared/screen/test/screenB.rs','useCache':true },

				];
				loadedRegion.render(screensInfo);
				//loadedRegion.setAnimation(0);
			}); */
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.renderRegion();
})
</script>
