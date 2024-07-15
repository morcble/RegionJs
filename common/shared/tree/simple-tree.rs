<style type="text/css">
.REGION_overlay{
	position: relative;
    height: 100%;
    z-index: 1;
}

#REGION{
	text-align: left !important;
}

</style>

<div id="REGION" class="hidden REGION_overlay" no-footer>
	<div class="tree" style="height:100%;padding-top: 0.125rem;box-sizing: border-box">
	
	</div>
	
	<div class="treemenu hidden" style="position:absolute">
			<table class="table table-bordered table-hover menutable">
				
			</table>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			this.accordion = this.paraMap.get("accordion");//accordion = 'true' 手风琴模式
			this.selectedNodeId = this.paraMap.get("selectedNodeId");//-1:不选中任何节点,   空值:选中第一个节点  , 节点ID:选中id指定的节点
			this.showDots = this.paraMap.get("showDots");// 虚线还是实线
			
			this.renderMenu = REGION.renderMenu.bind(this);
			this.addNode = REGION.addNode.bind(this);
			this.removeNode = REGION.removeNode.bind(this);
			this.refreshNode = REGION.refreshNode.bind(this);
			this.filterNode = REGION.filterNode.bind(this);
			
			var curRegion = this;
			curRegion.setTreeData = function(treeDataPara){
				curRegion.treeData = treeDataPara;
				curRegion.renderMenu(treeDataPara);
			}
			curRegion.setNodeClick = function(functionStrPara,para){
				var functionStr = "curRegion.nodeClick = function("+para+"){"+functionStrPara+"}";
				eval(functionStr);
			}

			this.minimize = function(){
				var tree = curRegion.find(".tree");
				curRegion.minimized = true;
				var children = tree.children();
				for(var i = 0 ;i<children.length;i++){
					REGION.minimizeMenuNode(children[i],curRegion);
				}
			};
			
			this.restore=function(){
				var tree = curRegion.find(".tree");
				curRegion.minimized = false;
				var children = tree.children();
				for(var i = 0 ;i<children.length;i++){
					REGION.restoreMenuNode(children[i],curRegion);
				}
			}
		},
		restoreMenuNode:function(menuNode,curRegion){//还原菜单
			var menuNodeObj = $(menuNode);
			var branchObj = menuNodeObj.find(".branch");
			branchObj.find(".menu-label").css("display","");
			branchObj.find(".toggleBtn").css("display","");
			menuNodeObj.find(".childs").css("display","");
			
			curRegion.find(".fa").unbind("mousemove");
			
			menuNodeObj.find(".icon-holder").css("position","static");
		},
		minimizeMenuNode:function(menuNode,curRegion){//缩小菜单
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
			var nodeHtml = "";
			var paddingLeft = 23;
			if(parentNodeData==null||parentNodeData.id=="0"){
				paddingLeft = 0;
			}

			var childs = nodeData.childs;
			if(childs!=null&&childs.length!=0){//绘制有child的节点
				var nodeClass = "menu-node";
				var opacity = 11-nodeData.depth;
				if(opacity<5)opacity=5;
				
				var currentNodeType="branch";
				
				nodeHtml="<div node-id='"+nodeData.id+"' onclick='REGION.selectNode(\""+nodeData.id+"\")' class='node_"+nodeData.id+" non-selected "+nodeClass+"'> ";
				if(this.showDots=="true")nodeHtml+='<div class="crossline-v" style="left:'+(2+paddingLeft*(nodeData.depth-1))+'px"></div>';
				
				nodeHtml+='<div class="pad">';
				if(nodeData.depth>1){
					nodeHtml+="<div style='background:white' class='"+currentNodeType+"'>";
				}
				else{
					nodeHtml+="<div class='first-level "+currentNodeType+"'>";
				}
				
		
				if(nodeData.depth>1){
					if(this.showDots=="true")
						nodeHtml+="<span class='lined' style='padding-left:"+(1+paddingLeft*(nodeData.depth-1))+"px'></span>";
					else	
						nodeHtml+="<span style='padding-left:"+(1+paddingLeft*(nodeData.depth-1))+"px'></span>";
				}
				else{
					nodeHtml+="<span style='padding-left:"+(1+paddingLeft*(nodeData.depth-1))+"px'></span>";
				}
				
				if(nodeData.expand)
					nodeHtml+='<i class="toggleBtn toggleTag fa fa-minus"></i>';
				else
					nodeHtml+='<i class="toggleBtn toggleTag fa fa-plus"></i>';
				
				if(nodeData.icon!=null)nodeHtml+="<span class='icon-holder'><i class='fa  "+nodeData.icon+"'></i></span>";
				nodeHtml+=this.nodeRender.call(this,nodeData);
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
	
				nodeHtml+="<div node-id='"+nodeData.id+"' onclick='REGION.selectNode(\""+nodeData.id+"\")' class='node_"+nodeData.id+" non-selected "+nodeClass+"'>";
				if(this.showDots=="true")nodeHtml+='<div class="crossline-v" style="left:'+(2+paddingLeft*(nodeData.depth-1))+'px"></div>';
				if(nodeData.depth>1){
					nodeHtml+="<div style='background:white' class='"+currentNodeType+"'>";
				}
				else{
					nodeHtml+="<div class='first-level "+currentNodeType+"'>";
				}
				nodeHtml+='<div class="pad">';
				if(nodeData.depth>1){
					if(this.showDots=="true")
						nodeHtml+="<span class='lined' style='padding-left:"+(1+paddingLeft*(nodeData.depth-1))+"px'></span>";
					else	 
						nodeHtml+="<span style='padding-left:"+(1+paddingLeft*(nodeData.depth-1))+"px'></span>";
				}
				else{
					nodeHtml+="<span style='padding-left:"+(1+paddingLeft*(nodeData.depth-1))+"px'></span>";
				}
				
				
				nodeHtml+='<i class="toggleTag fa fa-file-o"></i>';
				
				if(nodeData.icon!=null)nodeHtml+="<span class='icon-holder'><i class='fa  "+nodeData.icon+"'></i></span>";
				nodeHtml+=this.nodeRender.call(this,nodeData);
				nodeHtml+="</div>";
				nodeHtml+="</div>";
				nodeHtml+="</div>";
			}
			return nodeHtml;
		},
		resolveTreeData:function(curRegion,nodeDataList){//把节点list组装成tree  startDepth:开始构造树的depth,之前的节点全部忽略
			var selectedNodeId = curRegion.selectedNodeId;
			
			var nodeMap = new HashMap();
			curRegion.nodeMap=nodeMap;
			var rootNode = {"id":"0","depth":0,"label":"root node","childs":new Array(),"expand":true};//expand 是否展开子节点
			nodeMap.put(rootNode.id,rootNode);
			
			if(nodeDataList==null){
				return;
			}
			for(var i=0;i<nodeDataList.length;i++){
				if(selectedNodeId!=null && selectedNodeId == nodeDataList[i].id){
					nodeDataList[i].selected = true;
					nodeDataList[i].expand = true;//选中的节点 必然expanded
					curRegion.selectedNodeData = nodeDataList[i];
					curRegion.selectedNodeId = nodeDataList[i].id;
				}
				
				if(nodeDataList[i].seqNo==null)nodeDataList[i].seqNo=0;//默认给个排序号
				nodeMap.put(nodeDataList[i].id,nodeDataList[i]);
			}	
			
			var tmpParentNodeData;
			for(var i=0;i<nodeDataList.length;i++){
				tmpParentNodeData = nodeMap.get(nodeDataList[i].parentNodeId);
				if(tmpParentNodeData.childs==null){
					tmpParentNodeData.childs=new Array();
				}
				tmpParentNodeData.childs.push(nodeDataList[i]);
				nodeDataList[i].parent = tmpParentNodeData;
			}
			if(curRegion.nodeDataList!=null){
				curRegion.oldCachedNodeList = curRegion.cachedNodeList;//把之前缓存的数据移位
			}
			curRegion.cachedNodeList = nodeDataList;//把有效节点数据缓存 TODO 刷新优化
			
			//set depth
			REGION.initDepth(rootNode,1);
			
			//数据处理
			//子节点排序
			REGION.sortChilds(rootNode);//parentId为0的节点排序
			for(var i=0;i<nodeDataList.length;i++){
				//计算expand
				var tmpNode = nodeDataList[i];
				if(tmpNode.expand){
					var parent = tmpNode.parent;
					while(parent!=null){
						parent.expand = true;
						parent = parent.parent;
					}
				}
				
				//子节点排序
				REGION.sortChilds(nodeDataList[i]);
				
			}
			
			//如果没有默认选择节点,则直接选中第一个节点
			if(selectedNodeId==null&&selectedNodeId!="-1"&&rootNode.childs.length>0){
				var defaultSelectNodeData = rootNode.childs[0];
				
				defaultSelectNodeData.selected = true;
				defaultSelectNodeData.expand = true;//选中的节点 必然expanded
				curRegion.selectedNodeData = defaultSelectNodeData;
				curRegion.selectedNodeId = defaultSelectNodeData.id;
			}
			
			return rootNode;
		},
		sortChilds:function(nodeData){
			var childs = nodeData.childs;
			if(childs==null||childs.length==0)return;
			
			for(var i = 0 ; i<childs.length-1 ;i++){
				for(var j = i+1 ; j<childs.length ;j++){
					if(childs[j].seqNo>childs[i].seqNo){
						var tmp = childs[i] ;
						childs[i] = childs[j];
						childs[j] = tmp;
					}
				}
			}
		},
		initDepth:function(node,depth){
			if(node.childs!=null){
				for(var i=0;i<node.childs.length;i++){
					node.childs[i].depth = depth;
					REGION.initDepth(node.childs[i],depth+1);
				}
			}
		},
		renderMenu:function(treeListData){//绘制整个树节点
			this.rootNodeData = REGION.resolveTreeData(this,RegionUtil.clone(treeListData));
			
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
			var treeContentDiv = this.find(".tree");
			RegionUtil.setInnerHtml(this,treeContentDiv[0],this.treeHtml);	
			
			var curRegion = this;
			curRegion.find(".icon-holder>.fa").mouseover(function(){
				var msg = $(this).closest(".branch").find(".menu-label").text();
				
				if(curRegion.minimized){
					$(this.parentNode).css("min-width","200px");
					RegionUtil.showMsgForDomObj(this,msg,"right","#353d47","white",20,-5,-25,false); 
				}
			});
			
			curRegion.find(".icon-holder>.fa").mouseout(function(){
				if(curRegion.minimized){
					$(this.parentNode).css("min-width","0px");
					RegionUtil.clearMsgForDomObj(this); 
				}
			});
			
			if(curRegion.selectedNodeData!=null){
				REGION.subSelectNode(curRegion,curRegion.selectedNodeData,curRegion.find(".node_"+curRegion.selectedNodeData.id));
			}
			
			
			
			//右键菜单 begin
			var treeContentDiv = curRegion.find(".tree");
			
			if(curRegion.contextMenu!=null || curRegion.nodeMenu!=null){
				var y = getPos_Y(treeContentDiv[0]);
				//treeContentDiv.css("height",$(window).height()-y-50);				
				treeContentDiv.unbind("mousedown").bind("contextmenu", function (evt) {
		            evt.preventDefault();
		            
		            var eventTarget = RegionUtil.getEventTarget(evt);
		            var currentRegion = RegionUtil.getRegionByElem(eventTarget);
		            
					var rightClickOnNodeId = $(eventTarget).closest(".menu-node").attr("node-id");
					if(rightClickOnNodeId!=null){
						REGION.selectNodeInRegion(rightClickOnNodeId,currentRegion);
					}
					else{
						currentRegion.find(".branch-selected").removeClass("branch-selected");
						currentRegion.find(".leaf-selected").removeClass("leaf-selected");
						curRegion.find(".actived-node").removeClass("actived-node");
						//curRegion.find(".node-selected").removeClass("node-selected");
						currentRegion.selectedNodeId = null;
						currentRegion.selectedNodeData = null;
					}

		            REGION.showMenu(currentRegion);
		            var x = getPos_X(treeContentDiv[0])-treeContentDiv.width();
		            var y = getPos_Y(treeContentDiv[0]);
		          	
		            var menuObj = currentRegion.find(".treemenu");
		           
					var menuWidth = 130;//默认menu为150px 宽度
		            
					var positionLeft = evt.clientX-x;
					if(positionLeft>(treeContentDiv.width()-menuWidth) ){
						positionLeft = treeContentDiv.width()-menuWidth;
					}
					
					var positionTop = evt.clientY-y;
					
		            menuObj.css("left",positionLeft);
		            menuObj.css("top",positionTop);
		            currentRegion.contextMenuShown = true;
		            menuObj.removeClass("hidden");
		            
		            return false;
		        });
				
				treeContentDiv.click(function(evt){
					var currentRegion = RegionUtil.getRegionByEvent(event);
					REGION.hideMenu(currentRegion);
					if(currentRegion.selectedNodeId!=null){
						//clean node-selected class
						//currentRegion.find(".node-selected").removeClass("node-selected");
						currentRegion.selectedNodeId = null;
						currentRegion.selectedNodeData = null;
					}
				});
				
				var tempRegion = this;
				this.hideMenu = function(){
					REGION.hideMenu(tempRegion);
				}
			}	
			//右键菜单 end
		},
		hideMenu:function(currentRegion){
			if(currentRegion.contextMenuShown){
				var menuObj = currentRegion.find(".treemenu");
				menuObj.addClass("hidden");
				currentRegion.contextMenuShown = false;
			}
		},
		showMenu:function(currentRegion){
			var tableObject = currentRegion.find(".menutable");
			
			var renderred = tableObject.attr("renderred");
			tableObject.attr("renderred","0");
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
			
		},
		addNode:function(nodeData){//添加一个节点并渲染
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
			REGION.selectNodeInRegion(nodeData.id,this);
		},
		removeNode:function(nodeId){//删除一个节点并渲染
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
		refreshNode:function(nodeData){//刷新某个节点的数据，并渲染
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
		selectNodeInRegion:function(nodeId,curRegion){
			var nodeData = curRegion.nodeMap.get(nodeId);
			//toggle node begin
			var tmpElem = null;
			if(event==null){//非点击触发
				var nodeClass = ".node_"+nodeId;
				tmpElem = curRegion.find(nodeClass)[0];
			}
			else{//点击触发
				tmpElem = RegionUtil.getEventTarget(event);
			}
			REGION.subSelectNode(curRegion,nodeData,tmpElem);
			
			//recaculate length of corssline
			if(curRegion.showDots=="true")
				REGION.reCaculateLengthOfCrossLine(curRegion,curRegion.rootNodeData);
		},
		selectNode:function(nodeId){
			//点击节点
			var event = window.event || arguments.callee.caller.arguments[0];
			var curRegion = RegionUtil.getRegionByEvent(event);
			event.stopPropagation();
			REGION.selectNodeInRegion(nodeId,curRegion);
		},
		reCaculateLengthOfCrossLine:function(curRegion,nodeData){
			var childs = nodeData.childs;
			if(childs!=null){
				for(var i = 0 ; i <childs.length ; i++){
					if(i<(childs.length-1)){
						REGION.reCaculateLengthOfCrossLine(curRegion,childs[i]);
					}
					else {
						var lastBranchNode = childs[i];
						if(lastBranchNode.depth>1&&lastBranchNode.childs!=null&&lastBranchNode.childs.length>0){
							curRegion.find(".node_"+lastBranchNode.id).find(".crossline-v").css("height","40px");
						}
					}
				}
				
			}
		},
		subSelectNode:function(curRegion,nodeData,tmpElem){
			curRegion.selectedNodeData = nodeData;
			curRegion.selectedNodeId = nodeData.id;
			var menuObj = $(tmpElem).closest(".menu-node");//获取点击事件最近的节点
			
			//toggle node begin
			if($(tmpElem).hasClass("toggleBtn")){
				if(nodeData.childs!=null&&nodeData.childs.length>0){	
					var iconObj = menuObj.find(".toggleBtn").first();
					if(iconObj.hasClass("fa-minus")){
						iconObj.removeClass("fa-minus");
						iconObj.addClass("fa-plus");
					}
					else if(iconObj.hasClass("fa-plus")){
						iconObj.removeClass("fa-plus");
						iconObj.addClass("fa-minus");
					}
					
					var childsDivObj = menuObj.find(".childs").first();
					if(childsDivObj.hasClass("hidden")){
						childsDivObj.removeClass("hidden");
					}
					else{
						childsDivObj.addClass("hidden");
					}
				}	
			}
					
			//toggle node end
			curRegion.find(".actived-node").removeClass("actived-node");
			curRegion.find(".node_"+nodeData.id).addClass("actived-node");
			
			curRegion.find(".branch-selected").removeClass("branch-selected");//移除当前选中的节点的高亮
			if(nodeData.depth==1){
				menuObj.find(".branch").first().addClass("actived-node");
				//menuObj.find(".branch").first().addClass("branch-selected");
				if(curRegion.minimized){//如果是最小化菜单
					var rootLayout = RegionUtil.getRegionById("rootLayout");
					rootLayout.restore();
					
					var adminHeader = RegionUtil.getRegionById("adminheader");
					var navIcon = adminHeader.find(".fa-navicon");
					navIcon.removeClass("rotate-class2").addClass("rotate-class3");
					setTimeout(function(){
						navIcon.removeClass("rotate-class3");
					},200);
					
					//RegionUtil.clearMsgForRegion(curRegion);//去掉提示信息
				}
			}
			else if(nodeData.depth==2){
				if(nodeData.childs==null){
					curRegion.find(".leaf-selected").removeClass("leaf-selected");
					menuObj.find(".leaf").addClass("leaf-selected");
				}
			}
			else{
				if(nodeData.childs==null){
					curRegion.find(".leaf-selected").removeClass("leaf-selected");
					menuObj.find(".leaf").addClass("leaf-selected");
				}
			} 
			
			//高亮第一层branch
			while(menuObj.find(".first-level").length==0){
				menuObj = menuObj.parent().closest(".menu-node");
				if(menuObj.length==0)break;
			}
			
			var branchObj = menuObj.children(".branch");
			if(branchObj.length>0){
				branchObj.addClass("branch-selected");
			}
			else{
				menuObj.find(">.pad>.branch").addClass("branch-selected");
			}
			
			
			//手风琴
			if(curRegion.accordion=="true"){
				curRegion.find(".first-level").each(function(){
					if(!$(this).hasClass("branch-selected")){
						var tmpMenuObj = $(this).parent();
						tmpMenuObj.find(".childs").addClass("hidden");
						tmpMenuObj.find(".toggleBtn").removeClass("fa-minus").addClass("fa-plus"); 
					}
				});
			}
			
			//click
			if(curRegion.nodeClick!=null&&typeof(curRegion.nodeClick)==="function" ){
				REGION.hideMenu(curRegion);
				curRegion.nodeClick.call(curRegion,nodeData);
			}
		},
		filterNode:function(critea){
			console.log("filter nodes"+critea);
			
			if(critea.trim()==""){
				REGION.showNode(this.rootNodeData);
			}
			else {
				REGION.needHideNode(critea,this.rootNodeData);
			}
			console.log(this.rootNodeData)
			REGION.hideOrShowNode(this,this.rootNodeData);
		},
		needHideNode:function(critea,nodeData){
			var needHide = true;
			if(nodeData.label.indexOf(critea)!=-1){
				needHide = false;
			}
			
			if(nodeData.childs!=null&&nodeData.childs.length>0){
				for(var i = 0 ; i<nodeData.childs.length ;i++){
					var needHideChild = REGION.needHideNode(critea,nodeData.childs[i]);
					if(!needHideChild)needHide=false;//只要有一个子节点不需要隐藏,父节点就不需要隐藏
				}
				nodeData.hide = needHide;	
			}
			else{
				nodeData.hide = needHide;
			}
			
			return needHide;
		},
		hideOrShowNode:function(treeRegion,nodeData){
			if(nodeData.hide==true){
				treeRegion.find(".node_"+nodeData.id).addClass("hidden");
			}
			else{
				treeRegion.find(".node_"+nodeData.id).removeClass("hidden");
				if(nodeData.childs!=null&&nodeData.childs.length>0){
					for(var i = 0 ; i<nodeData.childs.length ;i++){
						REGION.hideOrShowNode(treeRegion,nodeData.childs[i]);
					}
				}
			}
		},
		showNode:function(nodeData){
			nodeData.hide = false;
			if(nodeData.childs!=null&&nodeData.childs.length>0){
				for(var i = 0 ; i<nodeData.childs.length ;i++){
					REGION.showNode(nodeData.childs[i]);
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