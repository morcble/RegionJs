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
	/*background: #f2f2f2;*/
	/*padding:10px;*/
	/*overflow: auto;*/
	/*height: calc(100% - 3.125rem);*/
}

</style>

<div id="REGION" class="hidden">
	<div class="nav-holder"></div>
	<div class="content-holder">
		<div class="skin content" style="height: 100%;background: white;position: relative">
		</div>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		unLoadTab:function(menuId){
			var regionId = this.regionId+"_tab_"+menuId;
			var tabRegion = RegionUtil.getRegionById(regionId);
			tabRegion.release();
			
			this.find(".content>.tab_"+menuId).remove();
			
			if(this.curMenuId==menuId){//当前tab被关闭
				if(this.preMenuId!=null){
					this.curMenuId = null;
					this.loadTab(this.preMenuId,null,null,true);
				}
			}
			else if(this.preMenuId==menuId){
				this.preMenuId=null;
			}
		},
		loadInShownTab:function(regionUrl){
			var region = this;
			var regionId = region.regionId+"_"+region.curTabId;
			console.log(regionId)
			var tabRegion = RegionUtil.getRegionById(regionId);
			tabRegion.release();
			
			//this.find(".content>.tab_"+menuId).remove();
			
			if(regionUrl==null)return;
			var tabObj = region.find(".tab[tab-id="+region.curTabId+"]");
			console.log(tabObj)
	
			RegionUtil.loadRegion(tabObj,regionUrl,this.regionId+"_"+region.curTabId);
		},
		loadTab:function(menuId,regionUrl,label,useCache){
			if(menuId==null)return;
			
			var region = this;
			var tabId = "tab_"+menuId;
			
  			if(useCache&&region.curTabId == tabId){
  				return;
  			}
			region.curTabId = tabId;
			
			var tabObj = region.find(".content>."+tabId);
		
			var tabExsit = false;
			region.find(".content>.tab").each(function(){
				var tmpTabId = this.getAttribute("tab-id");
				if(tmpTabId==tabId){
					tabExsit=true;
					$(this).removeClass("hidden");
					$(this).addClass("shown");
				}
				else{
					$(this).addClass("hidden");
					$(this).removeClass("shown");
				}
			});
			
			if(!tabExsit){
				if(regionUrl==null)return;
				var tabHtml = '<div rg-url="'+regionUrl+'" region-id="'+region.regionId+'" tab-id="'+tabId+'" class="tab tabinner'+tabId+' tab '+tabId+'"><div>';
				tabObj = $(tabHtml);
				RegionUtil.loadRegion(tabObj,regionUrl,this.regionId+"_"+tabId);
				region.find(".content").append(tabObj);
				
				region.navRegion.addNavItem(label,menuId);
			}
			else{
				if(!useCache && region.curTabId == tabId){
					var tabRegionId = this.regionId+"_"+tabId;
					var tabRegion = RegionUtil.getRegionById(tabRegionId);
					if(tabRegion!=null)tabRegion.release();
					
					
					var tabinner = region.find(".tabinner"+tabId);
					if(regionUrl==null){
						regionUrl = tabinner.attr("rg-url");
					}
					
					RegionUtil.loadRegion(region.find(".tabinner"+tabId),regionUrl,tabRegionId);
				}

				region.navRegion.selectItem(menuId);
			}
			
			region.preMenuId = region.curMenuId;//记录tab打开记录
			region.curMenuId = menuId;
			
			//RegionUtil.loadRegion(region.find(".content"),regionUrl);
		},
		renderContent:function(){
			var curRegion = this;
			var loadNavPromise = RegionUtil.loadRegion(this.find(".nav-holder"),"common/shared/index/screenStyle/nav.rs");
			loadNavPromise.done(function(navRegion){
				curRegion.navRegion = navRegion;
			});
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.loadTab = REGION.loadTab;
	staticRegion.unLoadTab = REGION.unLoadTab;
	staticRegion.loadInShownTab = REGION.loadInShownTab;
	staticRegion.renderRegion();
})
</script>

