function MobileSelect(){
		var mobileSelectDom=document.createElement("div");
		mobileSelectDom.className = "mobileSelect";
		var domInnerHtml = '<div class="grayLayer"></div>';
		domInnerHtml += '<div class="content">';
		domInnerHtml += '	<div class="btnBar">';
		domInnerHtml += '		<div class="fixWidth">';
		domInnerHtml += '			<div class="cancel">取消</div>';
		domInnerHtml += '			<div class="title"></div>';
		domInnerHtml += '			<div class="ensure">确认</div>';
		domInnerHtml += '		</div>';
		domInnerHtml += '	</div>';
		domInnerHtml += '	<div class="panel">';
		domInnerHtml += '		<div class="fixWidth">';
		domInnerHtml += '			<div class="wheels">';	
		domInnerHtml += '			</div>';
		domInnerHtml += '			<div class="selectLine"></div>';
		domInnerHtml += '			<div class="shadowMask"></div>';
		domInnerHtml += '		</div>';
		domInnerHtml += '	</div>';
		domInnerHtml += '</div>';
		
		mobileSelectDom.innerHTML = domInnerHtml;
		document.body.appendChild(mobileSelectDom);
		
		this.divObject = $(mobileSelectDom);
		this.divObject.find(".grayLayer").click(function(){
			mobileSelectDom.classList.remove('mobileSelect-show');
		})
		
		this.divObject.find(".cancel").click(function(){
			mobileSelectDom.classList.remove('mobileSelect-show');
		})
		
		var mobileSelectInstance = this;
		this.divObject.find(".ensure").click(function(){
			mobileSelectDom.classList.remove('mobileSelect-show');
			if (typeof mobileSelectInstance.choosenCallback === "function"){
				var wheelInstances = mobileSelectInstance.wheels;
				var result = [];
				for(var i = 0 ; i <wheelInstances.length;i++){
					result.push(wheelInstances[i].selectedOption);
				}
				mobileSelectInstance.choosenCallback(result,mobileSelectInstance.triggerDom);
			}
		})
	}
	
	//cascade模式下根据选中的值获取实际wheelsData
	MobileSelect.prototype.getCascadeWheelData = function(selectedValues){
		var wheelsData = [];
		var leftWheelData = null;
		for(var i = 0 ; i<this.cascadeDepth ;i++){
			var columnDatas = [];
			
			if(i==0){//第一列
				leftWheelData = this.treeWheelData;
				for(var j = 0 ; j <this.treeWheelData.length;j++){
					columnDatas.push(this.treeWheelData[j]);
				}
			}
			else{
				var wheelSelectedVal = selectedValues[i-1];//查看左边一列是否选值
				
				if(wheelSelectedVal==null){//没有选值
					wheelsData.push(columnDatas);
				}
				else{
					var currentWheelData = null;
					for(var j = 0 ; j <leftWheelData.length;j++){
						if(leftWheelData[j].value+""==wheelSelectedVal+""){
							currentWheelData = leftWheelData[j].childs;
							break;
						}
					}
					if(currentWheelData!=null){
						leftWheelData = currentWheelData;//给下一列使用
						for(var j = 0 ; j <currentWheelData.length;j++){
							columnDatas.push(currentWheelData[j]);
						}
					}
					else{
						leftWheelData = null;//给下一列使用
					}
				}
			}
			
			var wheelSelectedVal = selectedValues[i];
			if(columnDatas.length==0){//层级不够的情况
				selectedValues[i] = null;
			}
			else{
				if(wheelSelectedVal==null)selectedValues[i] = columnDatas[0].value;
			}
			
			wheelsData.push(columnDatas);
		}
		return wheelsData;
	}
	
	//刷新单列
	MobileSelect.prototype.refreshWheel = function(wheelIndex,wheelData,selectedValue){
		this.wheels[wheelIndex].refreshWith(wheelData,selectedValue);
	}
	
	//刷新所有列
	MobileSelect.prototype.refreshWheels = function(triggerDom,wheelsData,selectedValues,choosenCallback,transitionEndCallback,cascade,title,confirmText,cancelText){
		var mobileSelectInstance = this;
		//缓存参数
		wheelsData = RegionUtil.clone(wheelsData);
		this.wheelsData = wheelsData;
		this.triggerDom = triggerDom;
		this.cascade = cascade;
		this.choosenCallback = choosenCallback;
		
		if(title!=null)this.divObject.find(".title").html(title);
		if(confirmText!=null)this.divObject.find(".ensure").html(confirmText);
		if(cancelText!=null)this.divObject.find(".cancel").html(cancelText);
		
		if(this.cascade){
			var treeWheelData = [];
			this.treeWheelData = treeWheelData;
			var nodeMap = new HashMap();
			for(var i = 0 ; i<wheelsData.length ;i++){
				nodeMap.put(wheelsData[i].id,wheelsData[i]);
			
				if(wheelsData[i].parentNodeId=="0"||wheelsData[i].parentNodeId=="null"||wheelsData[i].parentNodeId==""){
					wheelsData[i].depth=1;
					treeWheelData.push(wheelsData[i]);
				}
			}
			wheelsData.sorted = true;
			
			for(var i = 0 ; i<wheelsData.length ;i++){
				var tmpParentNodeData = nodeMap.get(wheelsData[i].parentNodeId);
				if(tmpParentNodeData==null)continue;
				
				if(tmpParentNodeData.childs==null){
					tmpParentNodeData.childs=new Array();
				}
				wheelsData[i].depth = tmpParentNodeData.depth+1;
				tmpParentNodeData.childs.push(wheelsData[i]);
			}
			
			//计算层级深度
			var cascadeDepth = 0;//最大深度
			for(var i = 0 ; i<treeWheelData.length ;i++){
				var curDepth = RegionUtil.getDepthForTreeNode(treeWheelData[i]);
				if(curDepth>cascadeDepth)cascadeDepth=curDepth;
			}
			
			this.cascadeDepth = cascadeDepth;
			
			if(selectedValues==null)selectedValues=[];//以免空值报错
			
			this.wheelsData = this.getCascadeWheelData(selectedValues);
		}
		
		var wheels = this.divObject.find(".wheels");
		var wheelsHtml = "";
		for(var i = 0 ; i<this.wheelsData.length ;i++){
			wheelsHtml += '<div class="wheel wheel-'+i+'" style="width: 100%;">';
			wheelsHtml +='<ul class="selectContainer" style="transform: translate3d(0px, 0px, 0px);">';
			for(var j = 0 ; j <this.wheelsData[i].length;j++){
				wheelsHtml+='<li val="'+this.wheelsData[i][j].value+'">'+this.wheelsData[i][j].label+'</li>';
			}
			wheelsHtml+='</ul></div>';
		}			
		wheels.html(wheelsHtml);
		
		var wheels = this.divObject.find(".wheels");
		setTimeout(function(){
			mobileSelectInstance.wheels = [];
			wheels.children().each(function(index,wheel){
				var wheelInstance = null;
				if(selectedValues!=null){
					wheelInstance = new MobileSelectWheel(mobileSelectInstance,wheel,index,mobileSelectInstance.wheelsData[index],
							selectedValues[index],transitionEndCallback);
				}
				else{
					wheelInstance = new MobileSelectWheel(mobileSelectInstance,wheel,index,mobileSelectInstance.wheelsData[index],
							null,transitionEndCallback);
				}
				mobileSelectInstance.wheels.push(wheelInstance);
			});
		});
		setTimeout(function(){
			window.mobileSelect.divObject[0].classList.add('mobileSelect-show');
		})
	}

	function MobileSelectWheel(mobileSelectInstance,theWheel,index,wheelData,selectedValue,transitionEndCallback){
		this.domElem = theWheel;
		this.index = index;
		this.mobileSelectInstance = mobileSelectInstance;
		this.transitionEndCallback = transitionEndCallback;
		this.selectedValue = selectedValue;
		var mobileSelectWheelInstance = this;
		
		this.wheelData = wheelData;
		this.itemsCount = wheelData.length;
		theWheel.setAttribute("index",index);
		
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
		this.selectContainer = $(theWheel).children()[0];
		this.rowHeight = this.selectContainer.querySelector('li').offsetHeight;
		//记录开始位置
		this.itemsOffset = {};
    	if(this.selectContainer.style.transform){
    		this.itemsOffset.y = parseInt(this.selectContainer.style.transform.split(',')[1]);
    	}
    	else{
    		this.itemsOffset.y = parseInt(this.selectContainer.style.webkitTransform.split(',')[1]);
    	}
    	
    	this.curPosition = null;//记录每次移动的位置
    	
		var dragStart = function(){
			var event = event || window.event;
			theWheel.addEventListener(actionMoveStr, dragMove);
			theWheel.addEventListener(actionEndStr,  dragEnd);
			event.preventDefault();
			
			mobileSelectWheelInstance.eventStartPosition = {};			
			if(isPc){
				mobileSelectWheelInstance.eventStartPosition.top = event.clientY;
            }
            else{
            	mobileSelectWheelInstance.eventStartPosition.top = event.targetTouches[0].pageY;
            }	
		};
		var dragMove = function(){
			var event = event || window.event;
			event.preventDefault();
			var currentTop;
			if(isPc){
				currentTop = event.clientY;
            }
            else{
            	currentTop = event.targetTouches[0].pageY;
            }
			
			var offsetTop = currentTop - mobileSelectWheelInstance.eventStartPosition.top;
        	//移动位置
        	mobileSelectWheelInstance.curPosition = mobileSelectWheelInstance.itemsOffset.y+offsetTop;
        	mobileSelectWheelInstance.gotoPosition(mobileSelectWheelInstance.itemsOffset.y+offsetTop);
		};

		var dragEnd = function(){
			var event = event || window.event;
			event.preventDefault();
			theWheel.removeEventListener(actionMoveStr, dragMove);
			theWheel.removeEventListener(actionEndStr, dragEnd);

            mobileSelectWheelInstance.itemsOffset.y　= mobileSelectWheelInstance.curPosition;//记录最后的位置
            mobileSelectWheelInstance.fixPosition();
		};

		
		theWheel.addEventListener(actionStartStr,dragStart);
		this.selectValue(selectedValue);
	}
	
	//修正位置
	MobileSelectWheel.prototype.fixPosition = function(){
		var offsetIndex = Math.round((this.getStartPosition()-this.itemsOffset.y)/this.rowHeight);//选中的index
		var selectedIndex = this.selectIndex(offsetIndex);
	
		this.selectedValue = this.wheelData[selectedIndex].value;
		this.selectedOption = this.wheelData[selectedIndex];
		
		if(this.mobileSelectInstance.cascade){//级联刷新 刷新下一级
			var selectWheelInstances = this.mobileSelectInstance.wheels;
			if(this.index == selectWheelInstances.length-1){
				//最后一列的级联　不做刷新
			}
			else{
				var selectedValues = [];
				for(var i = 0 ; i<this.index+1 ;i++){
					selectedValues.push(selectWheelInstances[i].selectedValue);
				}
				
				//重新计算并赋值wheelsData
				console.log("recalculate wheelsData");
				this.mobileSelectInstance.wheelsData = this.mobileSelectInstance.getCascadeWheelData(selectedValues);
			
				for(var i = this.index+1;i<selectWheelInstances.length ;i++){
					selectWheelInstances[i].refreshWith(this.mobileSelectInstance.wheelsData[i],selectedValues[i]);
				}
			}
		}
		
		if (typeof this.transitionEndCallback === "function"){
			this.transitionEndCallback(selectedIndex,this.wheelData[selectedIndex],this.mobileSelectInstance.triggerDom);
		}
	}
	
	//传入数据和选中的值，刷新列及绑定事件
	MobileSelectWheel.prototype.refreshWith = function(wheelData,selectedValue){
		this.mobileSelectInstance.wheelsData[this.index] = wheelData;//如果是外部刷新,则需要同步数据
		
		this.wheelData = wheelData;
		this.itemsCount = wheelData.length;
		
		var wheelHtml = "";
		for(var j = 0 ; j <this.wheelData.length;j++){
			wheelHtml+='<li val="'+this.wheelData[j].value+'">'+this.wheelData[j].label+'</li>';
		}
		
		$(this.selectContainer).html(wheelHtml);//重新绘制dom
		if(selectedValue==null)selectedIndex=0;
		if(selectedIndex==null)selectedIndex=0;
		var selectedIndex = this.selectIndex(selectedIndex);
		
		if(this.wheelData.length>0){
			this.selectedValue = this.wheelData[selectedIndex].value;
			this.selectedOption = this.wheelData[selectedIndex];
		}
		else{
			this.selectedValue = null;
			this.selectedOption = null;
		}
	}
	
	/*
	*初始化加载的whell的偏移位
	*/
	MobileSelectWheel.prototype.getStartPosition = function(){
		return this.rowHeight*2;
	}
	
	MobileSelectWheel.prototype.gotoPosition = function(positionOffset){
		if(this.selectContainer.style.transform){
			this.selectContainer.style.transform = 'translate3d(0,' + positionOffset + 'px, 0)';
        }else{
        	this.selectContainer.style.webkitTransform = 'translate3d(0,' + positionOffset + 'px, 0)';
        } 
	}
	
	//选中第几个选项,从0开始
	MobileSelectWheel.prototype.selectIndex = function(indexNo){
		if(this.selectedIndex==indexNo)return indexNo;//避免重复执行
		
		if(indexNo==null){
			indexNo = Math.floor(this.itemsCount/2);	
		}	
		else if(indexNo<0)
			indexNo=0;
		else if(indexNo>=this.itemsCount)
			indexNo = this.itemsCount-1;

		var offsetPosition = this.getStartPosition()-indexNo*this.rowHeight;
		this.gotoPosition(offsetPosition);
		this.itemsOffset.y = offsetPosition;//存储位置
		this.selectedIndex = indexNo;
		return indexNo;
	}
	
	MobileSelectWheel.prototype.selectValue = function(selectedValue){
		var selectedIndex = null;
		for(var i = 0 ; i<this.wheelData.length ;i++){
			if(this.wheelData[i].value+""==selectedValue+""){
				selectedIndex = i;
				this.selectedOption = this.wheelData[i];
				break;
			}
		}
		
		return this.selectIndex(selectedIndex);
	}