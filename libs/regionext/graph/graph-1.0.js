(function ($) {
	if($.rsplugin==null)$.rsplugin={};
	
	var isPc = RegionUtil.isPc();
	var actionStartStr,actionMoveStr,actionEndStr;
	if(isPc){
		actionStartStr = "mousedown";
		actionMoveStr = "mousemove";
		actionEndStr = "mouseup";
	}
	else{
		actionStartStr = "touchstart";
		actionMoveStr = "touchmove";
		actionEndStr = "touchend";
	}
	
	
	$.rsplugin.graph={
		selectMode:"single",
		focusedBoard:null,//聚焦的board
		boardReg: {},//drawBoard注册表
		initDrawBoard:function(boardRegion){
			return new DrawBoard(boardRegion);
		},
		getBoard:function(shapeRegion){
			var boardId = shapeRegion.paraMap.get("boardId");
			return $.rsplugin.graph.boardReg[boardId];
		},
		systemShape:{
			"dot":"data:image/svg+xml;base64,PCFET0NUWVBFIHN2ZyBQVUJMSUMgIi0vL1czQy8vRFREIFNWRyAxLjEvL0VOIiAiaHR0cDovL3d3dy53My5vcmcvR3JhcGhpY3MvU1ZHLzEuMS9EVEQvc3ZnMTEuZHRkIj48c3ZnIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHdpZHRoPSIxOHB4IiBoZWlnaHQ9IjE4cHgiIHZlcnNpb249IjEuMSI+PGNpcmNsZSBjeD0iOSIgY3k9IjkiIHI9IjUiIHN0cm9rZT0iI2ZmZiIgZmlsbD0iIzI5YjZmMiIgc3Ryb2tlLXdpZHRoPSIxIi8+PC9zdmc+"
		},
		
	};
	//监听全局keydown
	$(document).keydown(function(event){
		var keyCode  = event.keyCode;
		if(keyCode==17){
			$.rsplugin.graph.selectMode = "multiple";//可同时选择多个shape
		}
	})

	//释放按钮
	$(document).keyup(function(event){
		var keyCode  = event.keyCode;
		if(keyCode==17){
			$.rsplugin.graph.selectMode = "single";//一次只能选择一个shape
		}
	})


	//画板
	function DrawBoard(boardRegion){
		var curBoard = this;
		this.id = RS.UUID();//唯一ID
		//this.shapes = {};//TODO 所有的shape
		this.selectedShapes = {};//选中的shapes
		
		this.relations = {};//所有shape的连线
		this.draftRelation = null;//正在连线的relation
		this.selectedRelations = {};//所有选中的的连线
		
		this.contextmenu="";//右键菜单
		
		$.rsplugin.graph.boardReg[this.id]=this;//加入注册表
		this.boardRegion = boardRegion;
		var boardDom = boardRegion.getRegionDivElem();
		var svgHtml = `
		<div class="boardmenu" style="position:absolute;width:12rem">
			<table class="table table-bordered table-hover">
				
			</table>
		</div>
		<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="100%" height="100%">
		 	<g class="shapes"></g>
		 	<g class="shadows"></g>
			<g class="relations"></g>
		</svg>`;//图形 高亮 路径
		RS.setInnerHtml(boardRegion,boardDom,svgHtml);

		boardDom.addEventListener(actionStartStr, function(event){
			$.rsplugin.graph.focusedBoard = curBoard;
		});
		
		$(boardDom).click(function(event){
			curBoard.hideMenu();
			var targetRegion = RS.getRegionByEvent(event);
			if(targetRegion==null || targetRegion.bizType!="shape"){
				//clear shape selection
				var shadows = curBoard.boardRegion.find(".shadows");
				shadows.html("");
				for(x in curBoard.selectedShapes){
					curBoard.selectedShapes[x].selected = false;
				}
			}
		})
		
		$(boardDom).mousemove(function(event){
			if(curBoard.status == "draftRelation"){
				var left,top;
				if(isPc){
	            	left = event.clientX;
	            	top = event.clientY;
	            }
	            else{
	            	left = event.targetTouches[0].pageX;
	            	top = event.targetTouches[0].pageY;
	            }
				var position = RS.getElemPosition(this);
				var relationCurrentPoint = {};
				relationCurrentPoint.left = left-position.x;
				relationCurrentPoint.top = top-position.y;
				curBoard.draftingRelation(curBoard.relationStartShape,curBoard.relationStartPoint,relationCurrentPoint,curBoard.relationType);
			}
		})
		
		//右键菜单
		$(boardDom).bind("contextmenu", function (evt) {
				curBoard.hideMenu();
				if(curBoard.status=="draftRelation"){
					//画线状态时，右键中断划线
					curBoard.status = null;
					var relationsHolder = curBoard.boardRegion.find(".relations");
					relationsHolder.find("[draft='1']").remove();
				}
	            evt.preventDefault();
	            var eventTarget = RegionUtil.getEventTarget(evt);
	            var eventTargetRegion = RegionUtil.getRegionByElem(eventTarget);

	            var boardObj =  $(boardDom);
	            var boardMenuObj = boardObj.find(".boardmenu");
	            var position = RS.getDomPosition(this);
		        
				var positionLeft = evt.clientX-position.x;
				var positionTop = evt.clientY-position.y;
	            boardMenuObj.css("left",positionLeft);
	            boardMenuObj.css("top",positionTop);
	            
	            var showMenu = false;
	            if(typeof(curBoard.onContextClick)==="function"){
	            	showMenu = curBoard.onContextClick(eventTargetRegion,eventTarget,{x:positionLeft,y:positionTop});
	            }
	            if(showMenu){
	            	RS.setInnerHtml(curBoard.boardRegion,boardMenuObj.find(".table")[0],curBoard.contextmenu);
		            boardMenuObj.removeClass("hidden");
	            }
		});
		
		curBoard.enableDrag();
	}
	
	//获取和目标shape建立relation的源头shape
	DrawBoard.prototype.getParentShape = function(targetShape){
		var curBoard = this;
		var result = [];
		for(tmpRelationId in curBoard.relations){
			var tmpRelation = curBoard.relations[tmpRelationId];
			if(tmpRelation.endShape.regionId==targetShape.regionId){
				result.push(tmpRelation.startShape);
			}
		}
		return result;
	}
	
	//获取配置数据
	DrawBoard.prototype.getData = function(){
		var curBoard = this;
		var svgObject = curBoard.boardRegion.find("svg");
		var result = {};
		result.shapes = [];
		result.relations = [];
		svgObject.children(".shapes").find("foreignObject").each(function(){
			var foreignObject = $(this);
			var shapeRegionId = foreignObject.attr("shape-region");
			var shapeRegion = RS.getRegionById(shapeRegionId);
			
			var tmpShapeData = {};
			tmpShapeData.shapeRegionId = shapeRegionId;
			tmpShapeData.x = foreignObject.attr("x");
			tmpShapeData.y = foreignObject.attr("y");
			tmpShapeData.width = foreignObject.attr("width");
			tmpShapeData.height = foreignObject.attr("height");
			tmpShapeData.shapeType = shapeRegion.shapeType;
			
			var regionPara = {};
			var paraMap = shapeRegion.paraMap;
			var paraKeySet = paraMap.keySet();
			
			for(var i = 0 ; i<paraKeySet.length ;i++){
				var key = paraKeySet[i];
				if(key=="region-id")continue;
				regionPara[key] = paraMap.get(key);
			}
			tmpShapeData.regionPara = regionPara;
			tmpShapeData.rcsFile = shapeRegion.rcsFile;
			
			result.shapes.push(tmpShapeData);
		})
				
		for(tmpRelationId in curBoard.relations){
			var tmpRelation = curBoard.relations[tmpRelationId];
			var relationData = {};
			relationData.endPoint = tmpRelation.endPoint;
			relationData.startPoint = tmpRelation.startPoint;
			relationData.startShapeId = tmpRelation.startShape.regionId;
			relationData.endShapeId = tmpRelation.endShape.regionId;
			result.relations.push(relationData);
		}
		return result;
	}
	
	//使用数据渲染drawboard
	DrawBoard.prototype.setData = function(drawBoardData){
		var curBoard = this;
		console.log(drawBoardData)
		var svgObject = curBoard.boardRegion.find("svg");

		var shapeRegionMapping = {};//需要替换regionId 以免重复，这个用于建立旧新映射

		//恢复shape
		var shapeHolder = svgObject.children(".shapes");
		var shapeDatas = drawBoardData.shapes;
		//等待加载信号
		var waitCountDef = new jQuery.Deferred();
		var waitCountDown = shapeDatas.length;
		for(var i = 0 ; i <shapeDatas.length ;i++){
			var shapeData = shapeDatas[i];
			var newShapeRegionId = RS.UUID();
			shapeRegionMapping[shapeData.shapeRegionId] = newShapeRegionId;//映射
			var shapePara = {shapeType:shapeData.shapeType,x:shapeData.x,y:shapeData.y,height:shapeData.height,width:shapeData.width}
	        var regionPara = shapeData.regionPara;
			var promise = curBoard.addShape(shapePara,RS.appendParaObjectToUrl(shapeData.rcsFile,regionPara),newShapeRegionId);
			promise.done(function(){
				waitCountDown--;
				if(waitCountDown==0)waitCountDef.resolve();
			})
		}
		
		waitCountDef.promise().done(function(){
			//恢复relation 数据
			var relationDatas = drawBoardData.relations;
			for(var i = 0 ; i <relationDatas.length ;i++){
				var relationData = relationDatas[i];
				
				relationData.startShape = RS.getRegionById(shapeRegionMapping[relationData.startShapeId]);
				relationData.endShape = RS.getRegionById(shapeRegionMapping[relationData.endShapeId]);
				delete relationData.startShapeId;
				delete relationData.endShapeId;
				var relationId = RS.UUID();
				relationData.id = relationId;
				curBoard.relations[relationId] = relationData;
				
				//恢复relation UI
				curBoard.drawRelation(relationData);
			}
			
			
		})
		
		
	}
	
	DrawBoard.prototype.hideMenu = function(){
		var curBoard = this;
		var boardDom = curBoard.boardRegion.getRegionDivElem();
		$(boardDom).find(".boardmenu").addClass("hidden");
	}
	
	
	//relationType  0直线 1折线 2曲线
	DrawBoard.prototype.draftingRelation = function(startShape,relationStartPoint,relationEndPoint,relationType){
		var curBoard = this;
		
		var draftRelation = {};
		draftRelation.id = RS.UUID();//唯一ID
		draftRelation.startShape = startShape;
		draftRelation.endShape = null;
		draftRelation.startPoint = relationStartPoint;
		draftRelation.endPoint = relationEndPoint;
		
		curBoard.draftRelation = draftRelation;
		
		//console.log(relationStartPoint,relationEndPoint);
		var relationsHolder = curBoard.boardRegion.find(".relations");
		var relationHtml = `<g relation="${draftRelation.id}" draft="1" region-id="${curBoard.boardRegion.regionId}">
								<line class="line" pointer-events="none" region-id="${curBoard.boardRegion.regionId}" 
									x1="${relationStartPoint.left}" y1="${relationStartPoint.top}"
									x2="${relationEndPoint.left}" y2="${relationEndPoint.top}" 
									stroke="black" stroke-width="1" />
								<circle class="start" pointer-events="none" cx="${relationStartPoint.left}" cy="${relationStartPoint.top}" region-id="${curBoard.boardRegion.regionId}" 
								r="4" fill="white" stroke="black" stroke-width="1"/>
								<circle class="end" pointer-events="none" cx="${relationEndPoint.left}" cy="${relationEndPoint.top}" region-id="${curBoard.boardRegion.regionId}" 
								r="4" fill="black" stroke="black" stroke-width="1"/>
							</g>`;
		relationsHolder.find("[draft='1']").remove();//移除之前的draft
		relationsHolder.html(relationsHolder.html()+relationHtml);
	}
	
	DrawBoard.prototype.updateRelation = function(relation,updatePointType){
		var curBoard = this;
		var relationObj = curBoard.boardRegion.find("[relation='"+relation.id+"']");
		if(updatePointType=="startPoint"){
			relationObj.children(".line").attr("x1",relation.startPoint.left).attr("y1",relation.startPoint.top);
			relationObj.children(".start").attr("cx",relation.startPoint.left).attr("cy",relation.startPoint.top);
		}
		else if(updatePointType=="endPoint"){
			relationObj.children(".line").attr("x2",relation.endPoint.left).attr("y2",relation.endPoint.top);
			relationObj.children(".end").attr("cx",relation.endPoint.left).attr("cy",relation.endPoint.top);
		}		
	}
	
	DrawBoard.prototype.drawRelation = function(relationData){
		var curBoard = this;
		var relationStartPoint = relationData.startPoint;
		var relationEndPoint = relationData.endPoint;
//		console.log(relationData)
		var relationsHolder = curBoard.boardRegion.find(".relations");
		var relationHtml = `<g relation="${relationData.id}" region-id="${curBoard.boardRegion.regionId}">
								<line class="line" pointer-events="none" region-id="${curBoard.boardRegion.regionId}" 
									x1="${relationStartPoint.left}" y1="${relationStartPoint.top}"
									x2="${relationEndPoint.left}" y2="${relationEndPoint.top}" 
									stroke="black" stroke-width="1" />
								<circle class="start" pointer-events="none" cx="${relationStartPoint.left}" cy="${relationStartPoint.top}" region-id="${curBoard.boardRegion.regionId}" 
								r="4" fill="white" stroke="black" stroke-width="1"/>
								<circle class="end" pointer-events="none" cx="${relationEndPoint.left}" cy="${relationEndPoint.top}" region-id="${curBoard.boardRegion.regionId}" 
								r="4" fill="black" stroke="black" stroke-width="1"/>
							</g>`;
		relationsHolder.html(relationsHolder.html()+relationHtml);
	}
	
	
	//shapePara 图形初始化参数
	//shapeRegionUrl 图形组件
	//shapeRegionId 指定regionId
	DrawBoard.prototype.addShape = function(shapePara,shapeRegionUrl,shapeRegionId){
		var board = this;
		var boardId = board.id;
		if(shapePara==null){
			console.log("shapePara is null")
			RS.alert("shapePara is null");
			return;
		}
		if(shapePara.shapeType==null){
			console.log("shapeType in shapePara is null")
			RS.alert("shapeType in shapePara is null");
			return;
		}
		
		
		shapePara.boardId = boardId;
		
		var shapesArea = this.boardRegion.find(".shapes");
		if(shapeRegionId==null)shapeRegionId = RS.UUID();
		var foreignObject = document.createElementNS('http://www.w3.org/2000/svg', 'foreignObject');
		foreignObject.classList.add('area'+shapeRegionId);
		foreignObject.setAttribute("region-id",this.boardRegion.regionId);
		foreignObject.setAttribute("shape-region",shapeRegionId);

		var gObject = document.createElementNS('http://www.w3.org/2000/svg', 'g');
		gObject.appendChild(foreignObject);
		this.boardRegion.find(".shapes")[0].appendChild(gObject);
		
		$(foreignObject).mouseover(function(event){
			var targetRegion = RS.getRegionByEvent(event);
			if(targetRegion!=null && targetRegion.bizType=="shape"){
				shapeRegion = targetRegion;
				if(shapeRegion.selected==true){
					board.shapeDragable = true;
				}
			}
		})
		$(foreignObject).mouseout(function(event){
			board.shapeDragable = false;
		})
		
		$(foreignObject).mousedown(function(event){
			var bizType = RS.getRegionByEvent(event).bizType;
			if(bizType==null){//点击边缘画线
				var left,top;
				if(isPc){
	            	left = event.clientX;
	            	top = event.clientY;
	            }
	            else{
	            	left = event.targetTouches[0].pageX;
	            	top = event.targetTouches[0].pageY;
	            }
				
				var shapeRegionId = $(this).attr("shape-region");
				var shapeRegion = RS.getRegionById(shapeRegionId);
				var position = RS.getElemPosition(board.boardRegion.getDom());
				var relationPoint = {};
				relationPoint.left = left-position.x;
				relationPoint.top = top-position.y;
				
				if(board.status==null){
					board.status = "draftRelation";//切换成画线状态
					board.relationType = 1;
					board.relationStartPoint = relationPoint;
					board.relationStartShape = shapeRegion;
				}
				else if(board.status=="draftRelation"){
					if(board.relationStartShape==shapeRegion)return;//自身不能连线
					
					board.status = null;//切换成无状态
					board.relationEndPoint = relationPoint;
					board.relationEndShape = shapeRegion;
			
					board.draftRelation.endPoint=relationPoint;
					board.draftRelation.endShape=shapeRegion;
					board.relations[board.draftRelation.id] = board.draftRelation;
					board.draftRelation = null;
					
					var relationsHolder = board.boardRegion.find(".relations");
					relationsHolder.find("[draft='1']").attr("draft",null).children().attr("pointer-events",null);//draft转正
				}
				
			}
		})
		
		var areaObj = this.boardRegion.find(".shapes").find(".area"+shapeRegionId);
		
		var jqueryDef = new jQuery.Deferred();
		var loadPromise = RS.loadRegion(areaObj,shapeRegionUrl,shapeRegionId);
		loadPromise.done(function(shapeRegion){
			shapeRegion.board = board;
			shapeRegion.bizType="shape";
			shapeRegion.shapeType = shapePara.shapeType;
			shapeRegion.selected = false;
			shapeRegion.orignalPara = shapePara;//暂存旧属性
			
			//计算高宽 start
			var x = shapePara.x;
			var y = shapePara.y;
			var width = shapePara.width;
			var height = shapePara.height;
			
			x = x==null?"100":x;
			y = y==null?"100":y;
			width = width==null?"120":width;
			height = height==null?"60":height;
			
			var holder = $(shapeRegion.getHolder());
			holder.attr("x",x);
			holder.attr("y",y);
			holder.attr("width",width);
			holder.attr("height",height);
			
			shapeRegion.properties = {};
			shapeRegion.properties.x=parseInt(x);
			shapeRegion.properties.y=parseInt(y);
			shapeRegion.properties.width=parseInt(width);
			shapeRegion.properties.height=parseInt(height);
			//计算高宽 end

			$(shapeRegion.getDom()).mousedown(function(event){
				var shapeRegion = RS.getRegionByEvent(event);
				if(!shapeRegion.selected)
					board.clickShape(shapeRegion);
				else 
					shapeRegion.readyToUnselect = true;//准备不选中,要根据后续动作判断
				//event.stopPropagation();
			})
			
			$(shapeRegion.getDom()).mouseup(function(event){
				if(shapeRegion.readyToUnselect==true){
					board.clickShape(shapeRegion);
				}
			})
			
			jqueryDef.resolve(foreignObject);
		});
		
		return jqueryDef.promise();
	}
	
	DrawBoard.prototype.deleteShape = function(shapeRegion){
		var curBoard = this;
		var svgObject = curBoard.boardRegion.find("svg");

		svgObject.children(".shapes").find("foreignObject").each(function(){
			var foreignObject = $(this);
			var shapeRegionId = foreignObject.attr("shape-region");
			if(shapeRegionId==shapeRegion.regionId){
				foreignObject.remove();
				return false;
			}
		})
		
		for(tmpRelationId in curBoard.relations){
			var tmpRelation = curBoard.relations[tmpRelationId];
			if(tmpRelation.startShape==null||tmpRelation.endShape==null||
					tmpRelation.startShape.regionId==shapeRegion.regionId || tmpRelation.endShape.regionId==shapeRegion.regionId){
				curBoard.deleteRelation(tmpRelation);
			}
		}
	}
	
	DrawBoard.prototype.deleteRelation = function(relation){
		var curBoard = this;
		var svgObject = curBoard.boardRegion.find("svg");
		svgObject.children(".relations").find("[relation="+relation.id+"]").remove();
		delete curBoard.relations[relation.id];
	}

	DrawBoard.prototype.clickShape = function(shapeRegion){
		//console.log("clickShape")
		var board = this;
		board.shapeDragable = true;
		$.rsplugin.graph.focusedBoard = this;
		
		var selectMode = $.rsplugin.graph.selectMode;
		if(selectMode == "single"){
			for(x in this.selectedShapes){
				this.selectedShapes[x].selected = false;
				delete this.selectedShapes[x];
			}
			this.selectedShapes = {};
			this.selectedShapes[shapeRegion.regionId] = shapeRegion;
			shapeRegion.selected = true;
		}
		else if(selectMode == "multiple"){
			var isSelectedExist = this.selectedShapes[shapeRegion.regionId];
			if(isSelectedExist!=null){
				delete this.selectedShapes[shapeRegion.regionId];
				shapeRegion.selected = false;
			}
			else{
				this.selectedShapes[shapeRegion.regionId] = shapeRegion;
				shapeRegion.selected = true;
			}
		}
		shapeRegion.readyToUnselect = false;

		var curHtml = "";
		for(tmpShapeRegionId in this.selectedShapes){
			var highlightedArray = this.calculateHighlightedForSelect(this.selectedShapes[tmpShapeRegionId]);
			for(var i = 0 ; i <highlightedArray.length ;i++){
				curHtml+=highlightedArray[i];
			}
		}
		var shadows = this.boardRegion.find(".shadows");
		shadows.html(curHtml);
	}
	
	DrawBoard.prototype.calculateHighlightedForSelect = function(shapeRegion){
		var result = [];
		var tmpProperties = shapeRegion.properties;
		let image = null;
		
		var minDotGap = 50;
		var dotSize = 16;

		//计算横向分割份数
		var hcount = Math.floor(tmpProperties.width/minDotGap);
		if(hcount%2 === 1)hcount=hcount-1;
		if(hcount<2)hcount=2;

		//横1
		for(var i = 0 ;i<(hcount+1);i++){
			image = `<image x="${tmpProperties.x-dotSize/2+i*tmpProperties.width/hcount}" y="${tmpProperties.y-dotSize/2}" width="${dotSize}" height="${dotSize}" 
				xlink:href="${$.rsplugin.graph.systemShape.dot}" preserveAspectRatio="none">
			</image>`;
			result.push(image);
		}
		//横2
		for(var i = 0 ;i<(hcount+1);i++){
			image = `<image x="${tmpProperties.x-dotSize/2+i*tmpProperties.width/hcount}" y="${tmpProperties.y-dotSize/2+tmpProperties.height}" width="${dotSize}" height="${dotSize}" 
				xlink:href="${$.rsplugin.graph.systemShape.dot}" preserveAspectRatio="none">
			</image>`;
			result.push(image);
		}
		
		
		//计算纵向分割份数
		var vcount = Math.floor(tmpProperties.height/minDotGap);
		if(vcount % 2 === 1)vcount=vcount-1;
		if(vcount==0)vcount=1;
		//纵1
		for(var i = 1 ;i<(vcount);i++){
			image = `<image x="${tmpProperties.x-dotSize/2}" y="${tmpProperties.y-dotSize/2+i*tmpProperties.height/vcount}" width="${dotSize}" height="${dotSize}" 
				xlink:href="${$.rsplugin.graph.systemShape.dot}" preserveAspectRatio="none">
			</image>`;
			result.push(image);
		}
		//纵2
		for(var i = 1 ;i<(vcount);i++){
			image = `<image x="${tmpProperties.x-dotSize/2+tmpProperties.width}" y="${tmpProperties.y-dotSize/2+i*tmpProperties.height/vcount}" width="${dotSize}" height="${dotSize}" 
				xlink:href="${$.rsplugin.graph.systemShape.dot}" preserveAspectRatio="none">
			</image>`;
			result.push(image);
		}

		return result;
	}
	
	DrawBoard.prototype.calculateHighlight = function(){
		var curHtml = "";
		for(x in this.selectedShapes){
			var highlightedArray = this.calculateHighlightedForSelect(this.selectedShapes[x]);
			for(var i = 0 ; i <highlightedArray.length ;i++){
				curHtml+=highlightedArray[i];
			}
		}
		var shadows = this.boardRegion.find(".shadows");
		shadows.html(curHtml);
	}
	
	//board drag
	DrawBoard.prototype.enableDrag = function(){
		var board = this;
		var boardRegion = board.boardRegion;
		var boardDom = boardRegion.getDom();
		var dragObj = $(boardDom);
		var left,top;
		var eventStartPosition;//鼠标事件开始坐标
		var dragStart = function(){
			//console.log("start")
			var shapeRegion = null;
			var targetRegion = RS.getRegionByEvent(event);
			if(targetRegion!=null && targetRegion.bizType=="shape"){
				shapeRegion = targetRegion;
			}
			else return;
			//board.clickShape(shapeRegion);
			
			if(board.shapeDragable!=true)return;
			
			boardDom.addEventListener(actionMoveStr, dragMove);
			boardDom.addEventListener(actionEndStr,  dragEnd);
            if(isPc){
            	left = event.clientX;
            	top = event.clientY;
            }
            else{
            	left = event.targetTouches[0].pageX;
            	top = event.targetTouches[0].pageY;
            }
            eventStartPosition = {};
            eventStartPosition.left = left;
            eventStartPosition.top = top;
		};
		var dragMove = function(){
			if(board.status!=null)return;//无状态的时候才能移动图形
			var left,top;
            if(isPc){
            	left = event.clientX;
            	top = event.clientY;
            }
            else{
            	left = event.targetTouches[0].pageX;
            	top = event.targetTouches[0].pageY;
            }
            
            for(x in board.selectedShapes){
            	var selectedShape = board.selectedShapes[x];
            	moveShape(selectedShape,left,top);
            }
        	
        	//实时计算highlight
			board.calculateHighlight();
		};
		
		//移动shape
		var moveShape = function(targetShape,left,top){
			var dragObj = $(targetShape.getHolder());
			if(targetShape.startedPosition==null){//准备工作
				targetShape.startedPosition = {};
				targetShape.startedPosition.left = parseInt(dragObj.attr("x"));
            	targetShape.startedPosition.top = parseInt(dragObj.attr("y"));
            	targetShape.startedPosition.width = parseInt(dragObj.attr("width"));
            	targetShape.startedPosition.height = parseInt(dragObj.attr("height"));
            	
            	//计算相关relation
            	targetShape.relatedRelations = {};
            	targetShape.relatedRelationsUpdateType = {};
            	targetShape.relatedRelationsRatio = {};
            	for(relationId in board.relations){
            		var relation = board.relations[relationId];
            		var needToUpdateRelationPoint = null;
            		var updatePointType = null;
            		if(relation.startShape==targetShape){
            			//need to update startpoint
            			updatePointType = "startPoint";
            			needToUpdateRelationPoint = relation.startPoint;
            		}
            		else if(relation.endShape==targetShape){
            			//need to update endpoint
            			updatePointType = "endPoint";
            			needToUpdateRelationPoint = relation.endPoint;
            		}
            		if(needToUpdateRelationPoint!=null){
            			//计算位置比例
            			var xRatio = (targetShape.startedPosition.left - needToUpdateRelationPoint.left)/targetShape.startedPosition.width;
            			var yRatio = (targetShape.startedPosition.top - needToUpdateRelationPoint.top)/targetShape.startedPosition.height;

                		targetShape.relatedRelations[relation.id] = relation;
                		targetShape.relatedRelationsUpdateType[relation.id] = updatePointType;
                		targetShape.relatedRelationsRatio[relation.id] = {"xRatio":xRatio.toFixed(5),"yRatio":yRatio.toFixed(5)};
            		}
            		
            	}
            }
            else{//移动时会一直触发
            	var offsetLeft = left - eventStartPosition.left;
            	var offsetTop = top - eventStartPosition.top;
            	var x = (targetShape.startedPosition.left+offsetLeft);
            	var y = (targetShape.startedPosition.top+offsetTop);
            	dragObj.attr("x",x);
            	dragObj.attr("y",y);
            	
            	targetShape.properties.x=parseInt(x);
            	targetShape.properties.y=parseInt(y);
            	//update relation location
            	for(relationId in targetShape.relatedRelations){
            		var relation = targetShape.relatedRelations[relationId];
            		//更新startPoint 或 endPoint
            		var updatePointType = targetShape.relatedRelationsUpdateType[relationId];
         
            		var ratio = targetShape.relatedRelationsRatio[relation.id];
           
            		if(updatePointType=="startPoint"){
            			relation.startPoint.left = x - ratio.xRatio*targetShape.startedPosition.width;
            			relation.startPoint.top = y - ratio.yRatio*targetShape.startedPosition.height;
            		}
            		else if(updatePointType=="endPoint"){
            			relation.endPoint.left = x - ratio.xRatio*targetShape.startedPosition.width;
            			relation.endPoint.top = y - ratio.yRatio*targetShape.startedPosition.height;
            		}
            		board.updateRelation(relation,updatePointType);
            	}
            }
			targetShape.readyToUnselect = false;
		}
		
		var dragEnd = function(){
			boardDom.removeEventListener(actionMoveStr, dragMove);
			boardDom.removeEventListener(actionEndStr, dragEnd);
            startedPosition = null;
            eventStartPosition = null;
            
            for(x in board.selectedShapes){
            	var selectedShape = board.selectedShapes[x];
            	selectedShape.startedPosition = null;
            }
		};
		
		boardDom.addEventListener(actionStartStr,dragStart);
	}
	
	
	

})(jQuery);

/*
图形
连线
键盘鼠标事件
导入导出

历史记录*/