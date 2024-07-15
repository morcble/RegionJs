<style type="text/css">
#REGION{

	position: relative;
    overflow: hidden;
}


#REGION.minimize .header-toggle{
	padding-left:3.4375rem;
	transition: padding-left .3s;
}
#REGION.minimize .header-toggle>.logo-section{
	width:3.4375rem;
	transition: width .3s;
}
#REGION.minimize .menu-toggle{
	padding-left: 3.4375rem;
	transition: padding-left .3s;
}
#REGION.minimize .menu-toggle>.menuHolder-box{
	width:3.4375rem;
	transition: width .3s;
}

#REGION.minimize .menu-toggle .menu-label{
	display: none;
}
#REGION.minimize  .childs{
	display: none;
}




</style>

<div id="REGION" class="hidden light-pot normal">
	<div class="header-holder"></div>
	<div class="contentwrapper-holder"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			themeUtil.theme();
		},
		minimize:function(){//缩小
			$(this.getRegionDivElem()).addClass("minimize").removeClass("normal");
		},
		restore:function(){//放大
			$(this.getRegionDivElem()).removeClass("minimize").addClass("normal");
		},
		init:function(treeRootNodeData){
			var curRegion = this;
			curRegion.treeRootNodeData = treeRootNodeData;
			var screenNodes = [];
			
			var header = curRegion.paraMap.get("header");
			if(header==null)header="common/shared/index/screenStyle/header.rs";
			
			if(treeRootNodeData!=null){
				var firstLevelChilds = treeRootNodeData.childs;
				var spark = new SparkMD5();
				for(var i = 0 ; i< firstLevelChilds.length ;i++){
					var tmp = {};
					var value = firstLevelChilds[i].value;
					if(value==null)value=firstLevelChilds[i].label;
					if(value.endWith(".rs")||value.indexOf(".rs?")!=-1){
						spark.append(value);
						var screenName = spark.end();
						
						firstLevelChilds[i].value = screenName;
						tmp.screenName = screenName;
						tmp.url=value;
						tmp.type="rs";//rs文件
					}
					else{
						tmp.screenName=value;
						tmp.url=value;
						tmp.type="plain";//普通节点
					}
					screenNodes.push(tmp);
				}
			}
			
			var paraMap = new HashMap();
			paraMap.put("screenNodes",screenNodes);
			
			var loadHeaderPromise = RegionUtil.loadRegion(curRegion.find(".header-holder"),header);
			var loadContentPromise = RegionUtil.loadRegion(curRegion.find(".contentwrapper-holder"),RS.appendParaMapToUrl("common/shared/index/screenStyle/screensHolder.rs",paraMap));
			$.when(loadHeaderPromise,loadContentPromise).done(function(headerRegion,contentRegion){
				if(treeRootNodeData!=null)headerRegion.setModuleItems(treeRootNodeData.childs);//加载导航按钮
				curRegion.headerRegion = headerRegion;
			})
		}
		
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.minimize = REGION.minimize;
	staticRegion.restore = REGION.restore;
	staticRegion.init = REGION.init;
	staticRegion.renderRegion();
})
</script>