var CoverPlugin = {
		interval:200,
		isCoverShown:false,
		showCount:0,
		checkThreadRuning:false,
		loadingStart:function(styleClass){
			if(!CoverPlugin.isCoverShown){
				CoverPlugin.subShowCover(styleClass);
				CoverPlugin.startCheckThread();
			}
			CoverPlugin.showCount++;
		},
		loadingComplete:function(){//准备隐藏cover
			if(CoverPlugin.showCount>0)
				CoverPlugin.showCount--;
		},
		startCheckThread:function(){
			CoverPlugin.startLoading = new Date().getTime();
			setTimeout(CoverPlugin.checkCover,CoverPlugin.interval);
		},
		checkCover:function(){
			if(CoverPlugin.showCount==0){
				CoverPlugin.subHideCover();
			}
			else{
				if(new Date().getTime()-CoverPlugin.startLoading>8000){//最多8秒
					CoverPlugin.showCount = 0;
					CoverPlugin.subHideCover();
				}
				else{
					setTimeout(CoverPlugin.checkCover,CoverPlugin.interval);
				}
			}
		},
		subShowCover:function(styleClass){
			console.log("showcover");
			var tmpLcd = CoverPlugin.getLoadingCoverDiv();
			var containerInfo = {
				x: 0,
				y: 0,
			};
			
			var temp = {
				p_margin: 0.4,
			};
			
			if (window.innerWidth) {
				containerInfo.x = window.innerWidth;
			} else if ((document.body) && (document.body.clientWidth)) {
				containerInfo.x = document.body.clientWidth;
			}
			
			if (window.innerHeight) {
				containerInfo.y = window.innerHeight;
			} else if ((document.body) && (document.body.clientHeight)) {
				containerInfo.y = document.body.clientHeight;
			}
			
			if(styleClass!=null)tmpLcd.addClass(styleClass);
			tmpLcd.attr("tp-class",styleClass);
			
			$(tmpLcd.children("div").get(0)).css("margin-top", containerInfo.y * temp.p_margin);
			$(tmpLcd.children("div").get(0)).removeClass("hidden");
			tmpLcd.removeClass("hidden");
			CoverPlugin.blockInput();
			CoverPlugin.isCoverShown = true;
		},
		subHideCover:function(){
			console.log("hidecover");
			
			var tmpLcd = CoverPlugin.getLoadingCoverDiv();
			
			var tpClass = tmpLcd.attr("tp-class");
			if(tpClass!=null){
				tmpLcd.removeClass(tpClass);
				tmpLcd.attr("tp-class",null);
			}
			
			$(tmpLcd.children("div").get(0)).addClass("hidden");
			tmpLcd.addClass("hidden"); 
			CoverPlugin.unBlockInput();
			
			CoverPlugin.isCoverShown = false;
		},
		getLoadingCoverDiv:function(){
			if(window._LCD==null){
				var spinDiv=document.createElement("i");
				spinDiv.className="fa fa-spinner fa-pulse fa-3x fa-fw";
				
				var loadingContentDiv=document.createElement("div");
				loadingContentDiv.className="dialog_progress hidden";
				loadingContentDiv.appendChild(spinDiv);
				
				var tmpLcd=document.createElement("div");
				tmpLcd.className="cover_div hidden";
				tmpLcd.appendChild(loadingContentDiv);
				document.body.appendChild(tmpLcd); 
				window._LCD = $(tmpLcd);
			}
			return window._LCD;
		},
		blockInput:function(){
			document.onkeydown = function(e) {
				var isie = (document.all) ? true : false;
				var key;
				var ev;
				if (isie) {//IE
					key = window.event.keyCode;
					ev = window.event;
				} else {//
					key = e.which;
					ev = e;
				}
				if (key == 9 || key == 13) {//IE
					if (isie) {
						ev.keyCode = 0;
						ev.returnValue = false;
					} else {//
						ev.which = 0;
						ev.preventDefault();
					}
				}
			};
		},
		unBlockInput:function(){
			document.onkeydown = null;
		}
}		



//loading cover end