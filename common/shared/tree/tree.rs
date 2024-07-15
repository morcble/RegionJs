<style type="text/css">
.REGION_NODE{
	font-size:14px;
	padding-bottom: 5px;
	padding-top: 2px;
}

.REGION_NODE span{
	padding-left: 5px;
}


.REGION_treeContent{
	padding-top:5px;
	padding-left:5px;
}

#REGION input[type='checkbox']{
    width: 16px;
    height: 16px;
    background-color: #fff;
   
    border: 1px solid #c9c9c9;
    border-radius: 2px;
    outline: none;
    margin-left: 5px;
    margin-bottom: -10px;
    vertical-align:-3px;
}

#REGION .node-selected{
	 background-color: gray;
}


</style>

<div id="REGION" class="hidden">
	<div class="REGION_treeContent">
	
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			this.renderTree = REGION.renderTree.bind(this);
		},
		defaultNodeRender:function(nodeData){
			return "<span>"+nodeData.label+"</span>";
		},
		getNodeFullHtml:function(nodeData,parentNodeData){//拼接node的整体html
			var nodeHtml = "";
			var paddingLeft = 33;
			if(parentNodeData.id=="0"){
				paddingLeft = 0;
			}
			
			var childs = nodeData.childs;
			if(childs!=null&&childs.length!=0){//绘制有child的节点
				var nodeClass = null;
				
				if(nodeData.depth>this.renderStartDepth){
					if(!parentNodeData.expand){
						nodeClass = "REGION_NODE hidden";//根子节点
						nodeData.expand = false;
					}
					else{
						nodeClass = "REGION_NODE";//根子节点
					}
				}
				else if(nodeData.depth==this.renderStartDepth){
					nodeClass = "REGION_NODE";//根子节点
				}
				
				nodeHtml="<div onclick='REGION.selectNode("+JSON.stringify(nodeData)+")' class='non-selected "+nodeClass+"' style='padding-left:"
							+paddingLeft+"px'><i onclick='REGION.toggleNode(this)' class='REGION_clickable non-selected icon fa ";
				if(nodeData.expand)
					nodeHtml+=" fa-minus-square-o ";
				else
					nodeHtml+=" fa-plus-square-o ";
				nodeHtml+="' aria-hidden='true'></i>";
		
				nodeHtml+=this.nodeRender.call(this,nodeData);
							
				for(var i = 0;i<childs.length;i++){//绘制子节点
					nodeHtml+=REGION.getNodeFullHtml.call(this,childs[i],nodeData);
				}
				nodeHtml+="</div>";
			}
			else{//绘制没有child的节点
				//paddingLeft+=11;
				var nodeClass = null;
				if(nodeData.depth>this.renderStartDepth){
					if(parentNodeData.expand){
						nodeClass = "REGION_NODE";
					}
					else{
						nodeClass = "REGION_NODE hidden";
					}
				}
				else if(nodeData.depth==this.renderStartDepth){
					nodeClass = "REGION_NODE";//根子节点
				}
				
				nodeHtml+="<div onclick='REGION.selectNode("+JSON.stringify(nodeData)+")' class='non-selected "+nodeClass+"' style='padding-left:"+paddingLeft+"px'>";
				nodeHtml+="<i class='fa fa-file-text-o' aria-hidden='true'></i>";
				nodeHtml+=this.nodeRender.call(this,nodeData);
				nodeHtml+="</div>";
			}
			
			return nodeHtml;
		},
		toggleNode:function(eventElem){
			var eventObject = $(eventElem);
			
			var nodeObj = $(eventObject).closest(".REGION_NODE");
			
			var iconObj = $(nodeObj.children().get(0));
			if(iconObj.hasClass("fa-plus-square-o")){
				iconObj.removeClass("fa-plus-square-o");
				iconObj.addClass("fa-minus-square-o");
			}
			else if(iconObj.hasClass("fa-minus-square-o")){
				iconObj.removeClass("fa-minus-square-o");
				iconObj.addClass("fa-plus-square-o");
			}
			
			var divHtml = iconObj.closest("div");
			var childs = divHtml.children();
			for(var i = 0 ; i <childs.length ; i++){
				if("DIV"==childs[i].tagName){
					var childObj = $(childs[i]);
					if(childObj.hasClass("hidden")){
						childObj.removeClass("hidden");
					}
					else{
						childObj.addClass("hidden");
					}
				}
			}
		},
		resolveTreeData:function(treeRegion,nodeDataList){//把节点list组装成tree  startDepth:开始构造树的depth,之前的节点全部忽略
			var nodeMap = new HashMap();
			var rootNode = {"id":"0","depth":0,"label":"root node","childs":new Array(),"expand":true};//expand 是否展开子节点
			nodeMap.put(rootNode.id,rootNode);
			
			if(nodeDataList==null){
				return;
			}
			for(var i=0;i<nodeDataList.length;i++){
				nodeMap.put(nodeDataList[i].id,nodeDataList[i]);
			}	
			
			var tmpParentNode;
			for(var i=0;i<nodeDataList.length;i++){
				tmpParentNode = nodeMap.get(nodeDataList[i].parentNodeId);
				if(tmpParentNode.childs==null){
					tmpParentNode.childs=new Array();
				}
				tmpParentNode.childs.push(nodeDataList[i]);
			}
			if(treeRegion.nodeDataList!=null){
				treeRegion.oldCachedNodeList = treeRegion.cachedNodeList;//把之前缓存的数据移位
			}
			treeRegion.cachedNodeList = nodeDataList;//把有效节点数据缓存 TODO 刷新优化
			
			//set depth
			REGION.initDepth(rootNode,1);
			
			return rootNode;
		},
		initDepth:function(node,depth){
			if(node.childs!=null){
				for(var i=0;i<node.childs.length;i++){
					node.childs[i].depth = depth;
					REGION.initDepth(node.childs[i],depth+1);
				}
			}
		},
		renderTree:function(treeListData){
			this.rootNodeData = REGION.resolveTreeData(this,treeListData);
			
			this.treeHtml = "";
			this.renderStartDepth = 0;

			if(this.nodeRender==null){
				RegionUtil.debug("warning : No node render defined, use defaultNodeRender");
				this.nodeRender = REGION.defaultNodeRender;
			}
			if(this.rootNodeData==null)return;
			var childs = this.rootNodeData.childs;
			if(childs!=null&&childs.length!=0){
				for(var i = 0;i<childs.length;i++){
					this.treeHtml += REGION.getNodeFullHtml.call(this,childs[i],this.rootNodeData);//渲染depth为1的节点
				}
			}
			var treeContentDiv = this.find(".REGION_treeContent");
			RegionUtil.setInnerHtml(this,treeContentDiv[0],this.treeHtml);
			
			//右键菜单
			if(this.contextMenu!=null || this.nodeMenu!=null){
				var y = getPos_Y(treeContentDiv[0]);
				treeContentDiv.css("height",$(window).height()-y-50);				
				treeContentDiv.unbind("mousedown").bind("contextmenu", function (event) {
					event.preventDefault();
		            
		            var currentRegion = RegionUtil.getRegionByEvent(event);
		            REGION.showMenu(currentRegion);
		            var x = getPos_X(treeContentDiv[0])-treeContentDiv.width();
		            var y = getPos_Y(treeContentDiv[0]);
		          	
		            var menuObj = currentRegion.find(".treemenu");
		            menuObj.css("left",evt.clientX-x);
		            menuObj.css("top",evt.clientY-y);
		            currentRegion.contextMenuShown = true;
		            menuObj.removeClass("hidden");
		            
		            return false;
		        });
				
				treeContentDiv.click(function(){
					var event = window.event || arguments.callee.caller.arguments[0];
					var currentRegion = RegionUtil.getRegionByEvent(event);
					REGION.hideMenu(currentRegion);
					
					if(currentRegion.selectedNodeId!=null){
						//clean node-selected class
						currentRegion.find(".node-selected").removeClass("node-selected");
						currentRegion.selectedNodeId = null;
						currentRegion.selectedNodeData = null;
					}
				});
				
				var tempRegion = this;
				this.hideMenu = function(){
					REGION.hideMenu(tempRegion);
				}
			}			
		},
		selectNode:function(nodeData){
			var event = window.event || arguments.callee.caller.arguments[0];
			var treeRegion = RegionUtil.getRegionByEvent(event);
			REGION.hideMenu(treeRegion);
			
			event.stopPropagation();
			
			
			var tmpElem = RegionUtil.getEventTarget(event);
			var tmpElemObj = $(tmpElem);
			if(treeRegion.selectedNodeId!=nodeData.id){
				if(tmpElemObj.hasClass("non-selected")){//不能被选中的标签
					return;
				}
				
				if(treeRegion.selectedNodeId!=null){
					//clean node-selected class
					treeRegion.find(".node-selected").removeClass("node-selected");
				}
				
				tmpElemObj.addClass("node-selected");
				treeRegion.selectedNodeId = nodeData.id;
				treeRegion.selectedNodeData = nodeData;
			}
			else{
				if(treeRegion.selectedNodeId!=null){
					//clean node-selected class
					treeRegion.find(".node-selected").removeClass("node-selected");
					treeRegion.selectedNodeId = null;
					treeRegion.selectedNodeData = null;
				}
			}
		},
		hideMenu:function(currentRegion){
			if(currentRegion.contextMenuShown){
				var menuObj = currentRegion.find(".treemenu");
				menuObj.addClass("hidden");
				currentRegion.contextMenuShown = false;
			}
		},
		showMenu:function(currentRegion){
			var tableObject = currentRegion.find(".REGION_menutable");
			var regionId = currentRegion.regionId;
			var menuHtml = "<tr region-id='"+regionId+"' style='cursor: pointer' onclick='part1'><td region-id='"+regionId+"' ><span region-id='"+regionId+"' >part2</span></td></tr>";
			
			var menuItems = currentRegion.contextMenu;
			if(currentRegion.selectedNodeId==null){
				if(currentRegion.menuType=="contextMenu"){
					return;
				}
				menuItems = currentRegion.contextMenu;
				currentRegion.menuType=="contextMenu";
			}
			else{
				if(currentRegion.menuType=="nodeMenu"){
					return;
				}
				menuItems = currentRegion.nodeMenu;
				currentRegion.menuType=="nodeMenu";
			}
			
			tableObject.empty();
			var itemHtml;
			if(menuItems!=null){
				for(var i = 0 ; i <menuItems.length ; i++){
					itemHtml = menuHtml.replace("part1",menuItems[i].onclick);
					itemHtml = itemHtml.replace("part2",menuItems[i].label);
					tableObject.append(itemHtml);
				}
			}
		}
		
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.afterRenderData = REGION.renderContent;
	formRegion.renderRegion();
})

</script>