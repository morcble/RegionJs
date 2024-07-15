<style type="text/css">
.REGION_overlay{
	position: relative;
	height: 100%;
	z-index: 10;
	overflow-x: hidden !important;
	padding-bottom: 0 !important;
}

</style>

<div id="REGION" class="hidden REGION_overlay">
	<div class="stackmenu"></div>


</div>


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			this.accordion = this.paraMap.get("accordion");//accordion = 'true' 手风琴模式
			this.selectedNodeId = this.paraMap.get("selectedNodeId");//-1:不选中任何节点,   空值:选中第一个节点  , 节点ID:选中id指定的节点
			this.selectedNodeValue = this.paraMap.get("selectedNodeValue");
			
			this.renderMenu = REGION.renderMenu.bind(this);
			this.refresh = REGION.refresh.bind(this);
			this.addNode = REGION.addNode.bind(this);
			this.removeNode = REGION.removeNode.bind(this);
			this.refreshNode = REGION.refreshNode.bind(this);
			var currRegion = this;
			this.minimize = function(){
				var stackmenu = currRegion.find(".stackmenu");
				currRegion.minimized = true;
				var children = stackmenu.children();
				for(var i = 0 ;i<children.length;i++){
					REGION.minimizeMenuNode(children[i],currRegion);
				}
			};
			
			this.restore=function(){
				var stackmenu = currRegion.find(".stackmenu");
				currRegion.minimized = false;
				var children = stackmenu.children();
				for(var i = 0 ;i<children.length;i++){
					REGION.restoreMenuNode(children[i],currRegion);
				}
			}
			
			var eventListener = RS.newListener(function(eventData){
				var curRegion = RS.getRegionById(this.regionId);
				if(!$(curRegion.getRegionDivElem()).is(':visible'))return;//如果菜单是隐藏的 则不响应事件
				
				var tmpMenuNodeData = curRegion.nodeMap.get(eventData);
				REGION.subSelectNode(curRegion,tmpMenuNodeData);
			});
			this.addEventListener("clickMenuNode",eventListener);//监听点击menu事件
		},
		restoreMenuNode:function(menuNode,currRegion){//还原菜单
			var menuNodeObj = $(menuNode);
			var branchObj = menuNodeObj.find(".branch");
			branchObj.find(".menu-label").css("display","");
			branchObj.find(".toggleBtn").css("display","");
			menuNodeObj.find(".childs").css("display","");
			
			currRegion.find(".fa-solid").unbind("mousemove");
			
			menuNodeObj.find(".icon-holder").css("position","static");
		},
		minimizeMenuNode:function(menuNode,currRegion){//缩小菜单
			var menuNodeObj = $(menuNode);
			var branchObj = menuNodeObj.find(".branch");
			branchObj.find(".menu-label").css("display","none");
			branchObj.find(".toggleBtn").css("display","none");
			menuNodeObj.find(".childs").css("display","none");
			
			menuNodeObj.find(".icon-holder").css("position","fixed");
		},
		defaultNodeRender:function(nodeData){
			return "<span class='menu-label'>"+nodeData.label+"</span>";
		},
		getNodeFullHtml:function(nodeData,parentNodeData){//拼接node的整体html
			var nodeHtml = "",
				paddingLeft = 10;
			var fontSize=document.documentElement.style.fontSize.split("px")[0]
			if(parentNodeData.id=="0"){
				paddingLeft = 0;
			}
			
			var childs = nodeData.childs;
			if(childs!=null&&childs.length!=0){//绘制有child的节点
				var nodeClass = "menu-node";
				var opacity = 11-nodeData.depth;
				if(opacity<5)opacity=5;
				
				var currentNodeType="branch";
				
				nodeHtml="<div onclick='REGION.selectNode(\""+nodeData.id+"\")' class='node_"+nodeData.id+" non-selected "+nodeClass+"'> ";
				
				if(nodeData.depth>1){
					nodeHtml+="<div class='child-level "+currentNodeType+"'>";
				}
				else{
					nodeHtml+="<div class='first-level "+currentNodeType+"'>";
				}
				
		
				
				nodeHtml+="<span style='padding-left:"+((1+paddingLeft*(nodeData.depth-1)) / fontSize)+"rem'></span>";
				if(nodeData.icon!=null)nodeHtml+="<span class='icon-holder'><i class='fa-solid "+nodeData.icon+"'></i></span>";
				nodeHtml+=this.nodeRender.call(this,nodeData);
				nodeHtml+="<i class='toggleBtn non-selected fa-solid fa-lg";
				if(nodeData.expand)
					nodeHtml+=" fa-sort-up ";
				else
					nodeHtml+=" fa-sort-down ";
				nodeHtml+="' aria-hidden='true'></i>";
				nodeHtml+="</div>";
				
				if(nodeData.expand)
					nodeHtml+="<div class='childs'>";	
				else
					nodeHtml+="<div class='childs hidden'>";	
				for(var i = 0;i<childs.length;i++){//绘制子节点
					nodeHtml+=REGION.getNodeFullHtml.call(this,childs[i],nodeData);
				}
				nodeHtml+="</div>";
				nodeHtml+="</div>";
			}
			else{//绘制没有child的节点
				//paddingLeft+=11;
				var nodeClass = "menu-node";
				var opacity = 11-nodeData.depth;
				if(opacity<5)opacity=5;
				
				var currentNodeType=null;

				if(nodeData.depth==1){
					currentNodeType=" branch";
				}
				else{
					currentNodeType=" leaf";
				}
	
				nodeHtml+="<div onclick='REGION.selectNode(\""+nodeData.id+"\")' class='node_"+nodeData.id+" non-selected "+nodeClass+"'>";
				
				if(nodeData.depth>1){
					nodeHtml+="<div  class='child-level "+currentNodeType+"'>";
				}
				else{
					nodeHtml+="<div class='first-level "+currentNodeType+"'>";
				}
				nodeHtml+="<span></span>";
				if(nodeData.icon!=null)nodeHtml+="<span class='icon-holder'><i class='fa-solid "+nodeData.icon+"'></i></span>";
				nodeHtml+=this.nodeRender.call(this,nodeData);
				nodeHtml+="</div>";
				nodeHtml+="</div>";
			}
			return nodeHtml;
		},
		resolveTreeData:function(currRegion,nodeDataList){//把节点list组装成tree  startDepth:开始构造树的depth,之前的节点全部忽略
			if(nodeDataList==null)return;
			var treeData = RS.convertToTreeData(nodeDataList);
			var selectedNodeId = currRegion.selectedNodeId;
			var selectedNodeValue = currRegion.selectedNodeValue;
			currRegion.nodeMap = treeData[1];
			var rootNode = treeData[0];
			
			//获取选中节点
			var selectNode = null;
			if(selectedNodeId!=null){
				selectNode = currRegion.nodeMap.get(selectedNodeId);
			}
			
			if(selectNode==null&&selectedNodeValue!=null){
				for(var i=0;i<nodeDataList.length;i++){
					if(nodeDataList[i].value==selectedNodeValue){
						selectNode = nodeDataList[i];
						break;
					}
				}
			}
			
			if(selectNode==null){
				selectNode = rootNode.childs[0];
				if(selectNode!=null){
					while((selectNode.childs!=null&&selectNode.childs.length>0)){
						selectNode = selectNode.childs[0];
					}
				}
				
			}
			if(selectNode!=null){
				selectNode.expand = true;
				selectNode.selected = true;
				currRegion.selectedNodeData = selectNode;
				currRegion.selectedNodeId = selectNode.id;
				
				//展开父节点
				var parentNode = selectNode.parent;
				while((parentNode!=null)){
					parentNode.expand = true;
					parentNode = parentNode.parent;
				}
			}
			return rootNode;
		},
		renderMenu:function(treeListData,startDepth){
			this.rootNodeData = REGION.resolveTreeData(this,RegionUtil.clone(treeListData),startDepth);
			
			this.treeHtml = "";

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
			var treeContentDiv = this.find(".stackmenu");
			RegionUtil.setInnerHtml(this,treeContentDiv[0],this.treeHtml);	
			
			var currRegion = this;
			currRegion.find(".icon-holder>.fa-solid").mouseover(function(){
				var msg = $(this).closest(".branch").find(".menu-label").text();
				
				if(currRegion.minimized||window.minimizedAdmin){
					var paraMap = new HashMap();
					paraMap.put("msg",msg);
					RegionUtil.showMsg(this,RegionUtil.appendParaMapToUrl("common/shared/msgbox/simpleMsgBox.rs",paraMap),null,-10,40);
				}
			});
			
			currRegion.find(".icon-holder>.fa-solid").mouseout(function(){
				if(currRegion.minimized||window.minimizedAdmin){
					RegionUtil.clearAllMsg(); 
				}
			});
			
			if(currRegion.selectedNodeData!=null){
				REGION.subSelectNode(currRegion,currRegion.selectedNodeData,currRegion.find(".node_"+currRegion.selectedNodeData.id));
			}
		},
		addNode:function(nodeData){
			var parentNodeData = this.nodeMap.get(nodeData.parentNodeId);
			if(parentNodeData==null){
				RegionUtil.alert("Parent node not found with parentNodeId:"+nodeData.parentNodeId);
				return;
			}
			
			var childs = parentNodeData.childs ;//=new Array();
			
			if(parentNodeData.childs==null)
				parentNodeData.childs = new Array();
			
			parentNodeData.expand = true;
			parentNodeData.childs.push(nodeData);
			nodeData.parent = parentNodeData;
			nodeData.depth = parentNodeData.depth+1;
			this.nodeMap.put(nodeData.id,nodeData);
			
			REGION.sortChilds(parentNodeData);
			
			this.refreshNode(parentNodeData);
		},
		removeNode:function(nodeId){
			nodeId = nodeId+"";
			var nodeData = this.nodeMap.remove(nodeId);
			if(nodeData==null){
				RegionUtil.alert("Node not found with id:"+nodeId);
				return;
			}
			
			var parentNodeData = this.nodeMap.get(nodeData.parentNodeId);
			if(parentNodeData==null){
				RegionUtil.alert("Parent node not found with parentNodeId:"+nodeData.parentNodeId);
				return;
			}
			
			for(var i = 0;i<parentNodeData.childs.length;i++){
				if(parentNodeData.childs[i].id==nodeId){
					parentNodeData.childs = parentNodeData.childs.slice(i+1);
					break;
				}
			} 
			
			this.find(".node_"+nodeId).remove();
			//this.refreshNode(parentNodeData);
			
		},
		refreshNode:function(nodeData){
			if(nodeData.id=="0"){//刷新根节点
				this.treeHtml = "";
				
				var childs = nodeData.childs;
				if(childs!=null&&childs.length!=0){
					for(var i = 0;i<childs.length;i++){
						this.treeHtml += REGION.getNodeFullHtml.call(this,childs[i],nodeData);//渲染depth为1的节点
					}
				}
				var treeContentDiv = this.find(".tree");
				RegionUtil.setInnerHtml(this,treeContentDiv[0],this.treeHtml);
			}
			else{//刷新其它节点
				var replaceHtml = REGION.getNodeFullHtml.call(this,nodeData,nodeData.parent);
				var replaced = $(replaceHtml);
				RegionUtil.addRegionUniqueId(replaced[0],this.regionId);
				var nodeObj = this.find(".node_"+nodeData.id);
				nodeObj.replaceWith(replaced);
			}
		},
		refresh:function(nodeData){
			
		},
		selectNode:function(nodeId){
			var event = window.event || arguments.callee.caller.arguments[0];
			var currRegion = RegionUtil.getRegionByEvent(event);
			event.stopPropagation();
			var nodeData = currRegion.nodeMap.get(nodeId);
			var tmpElem = RegionUtil.getEventTarget(event);
			REGION.subSelectNode(currRegion,nodeData,tmpElem);
		},
		subSelectNode:function(currRegion,nodeData){
			currRegion.selectedNodeData = nodeData;
			//var menuObj = $(tmpElem).closest(".menu-node");
			var menuObj = currRegion.find(".node_"+nodeData.id);
			if(nodeData.childs!=null&&nodeData.childs.length>0){		
				var iconObj = menuObj.find(".toggleBtn").first();
				if(iconObj.hasClass("fa-sort-down")){
					iconObj.removeClass("fa-sort-down");
					iconObj.addClass("fa-sort-up");
				}
				else if(iconObj.hasClass("fa-sort-up")){
					iconObj.removeClass("fa-sort-up");
					iconObj.addClass("fa-sort-down");
				}
				
				var childsDivObj = menuObj.find(".childs").first();
				if(childsDivObj.hasClass("hidden")){
					childsDivObj.removeClass("hidden");
				}
				else{
					childsDivObj.addClass("hidden");
				}
			}			
			//toggle node end
			currRegion.find(".branch-selected").removeClass("branch-selected");
			currRegion.find(".leaf-selected").removeClass("leaf-selected");
			if(nodeData.depth==1){
				//menuObj.find(".branch").first().addClass("branch-selected");
				if(currRegion.minimized||window.minimizedAdmin){//兼容旧的框架页面
					var rootLayout = RegionUtil.getRegionById("rootLayout");
					if(rootLayout!=null){//旧的admin框架页面
						rootLayout.restore();
						var adminHeader = RegionUtil.getRegionById("adminheader");
						var navIcon = adminHeader.find(".fa-navicon");
						navIcon.removeClass("rotate-class2").addClass("rotate-class3");
						setTimeout(function(){
							navIcon.removeClass("rotate-class3");
						},200);
						
						
					}
					else{//新的框架页面 还原大小
						// var rscloudMainRegion = RS.getRegionById("rscloud_main");
						var rscloudMainRegion = RS.getOuterRegion(RS.getOuterRegion(currRegion))
						rscloudMainRegion.restore();
					}
					
					RegionUtil.clearAllMsg();
				}
			}
			else if(nodeData.depth==2){
				if(nodeData.childs==null){
					currRegion.find(".leaf-selected").removeClass("leaf-selected");
					menuObj.find(".leaf").addClass("leaf-selected");
				}
			}
			else{
				if(nodeData.childs==null){
					currRegion.find(".leaf-selected").removeClass("leaf-selected");
					menuObj.find(".leaf").addClass("leaf-selected");
				}
			} 
			
			//高亮第一层branch
			while(menuObj.find(".first-level").length==0){
				menuObj = menuObj.parent().closest(".menu-node");
				if(menuObj.length==0)break;
			}
			menuObj.children(".branch").addClass("branch-selected");
			
			//手风琴
			if(currRegion.accordion=="true"){
				currRegion.find(".first-level").each(function(){
					if(!$(this).hasClass("branch-selected")){
						var tmpMenuObj = $(this).parent();
						tmpMenuObj.find(".childs").addClass("hidden");
						tmpMenuObj.find(".toggleBtn").removeClass("fa-sort-up").addClass("fa-sort-down"); 
					}
				});
			}
			
			//click  模拟菜单选中事件
			if(currRegion.nodeClick!=null&&typeof(currRegion.nodeClick)==="function" ){
				currRegion.nodeClick.call(currRegion,nodeData);
			}
		}
		
		
};


RegionUtil.ready(function(){
	var formRegion = RegionUtil.newFormRegion("#REGION");
	formRegion.afterRenderData = REGION.renderContent;
	formRegion.renderRegion();
})

</script>