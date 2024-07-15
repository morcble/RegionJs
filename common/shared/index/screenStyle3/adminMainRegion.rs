<style type="text/css">
	#REGION{

		position: relative;
		overflow: hidden;
	}


	#REGION.minimize .header-toggle{
		padding-left:3.4375rem;
	}
	#REGION.minimize .header-toggle>.logo-section{
		width:3.4375rem;
	}
	#REGION.minimize .menu-toggle{
		padding-left: 3.4375rem;
	}
	#REGION.minimize .menu-toggle>.menuHolder-box{
		width:3.4375rem;
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
			//curRegion.treeRootNodeData = treeRootNodeData;
			var screenNodes = [];
			
			var header = curRegion.paraMap.get("header");
			if(header==null)header="common/shared/index/screenStyle3/header.rs";
			
			//给treeNodeData再添加一级父节点
			treeRootNodeData = REGION.refactNode(treeRootNodeData);
			curRegion.treeRootNodeData = treeRootNodeData;
			console.log(treeRootNodeData)
			
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
			var loadContentPromise = RegionUtil.loadRegion(curRegion.find(".contentwrapper-holder"),RS.appendParaMapToUrl("common/shared/index/screenStyle3/screensHolder.rs",paraMap));
			$.when(loadHeaderPromise,loadContentPromise).done(function(headerRegion,contentRegion){
				//headerRegion.setModuleItems(treeRootNodeData.childs);//加载导航按钮
				curRegion.headerRegion = headerRegion;
			})
		},
		refactNode:function(nodeData){
			if(nodeData.depth==0){
				var newRootNode = {};
				newRootNode.depth=0;
				newRootNode.expand=true;
				newRootNode.id="0";
				newRootNode.label="virtualroot";
				newRootNode.childs = [nodeData];
				
				nodeData.parentNodeId="0";
				nodeData.parent=newRootNode;
				nodeData.label="root";
				nodeData.id="99999999";
				nodeData.depth++;
				
				var childs = nodeData.childs;
				if(childs!=null){
					for(var i=0;i<childs.length;i++){
						REGION.refactNode(childs[i]);
					}
				}
				return newRootNode;
			}
			else{
				nodeData.parentNodeId=nodeData.parent.id;
				nodeData.depth++;
				var childs = nodeData.childs;
				if(childs!=null){
					for(var i=0;i<childs.length;i++){
						REGION.refactNode(childs[i]);
					}
				}
			}
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