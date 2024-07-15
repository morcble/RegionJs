var AlertPlugin = {
//		promiseQueue:new Array(),//等待队列
//		deferredQueue:new Array(),//等待队列
		msgQueue:new Array(),//等待队列
		toastQueue:new Array(),//信息队列
		consumeMsgQueue:function(){
			if(AlertPlugin.msgQueue.length>0){
				var msgObj = AlertPlugin.msgQueue.pop();
				if(msgObj.type==1){
					var paraMap = new HashMap();
					paraMap.put("msg",msgObj.msg);
					if(RegionUtil.isPc())
						AlertPlugin.showMsg(msgObj.title,380,200,"common/shared/popup/alert.rs",paraMap);
					else
						AlertPlugin.showMsg(msgObj.title,350,200,"common/shared/popup/alert.rs",paraMap);
					AlertPlugin.msgDef = msgObj.msgDef;
					AlertPlugin.msgPromise = msgObj.msgPromise;
				}
				else if(msgObj.type==2){
					AlertPlugin.yesCallBack = msgObj.yesCallBack;
					AlertPlugin.noCallBack = msgObj.noCallBack;
					var paraMap = new HashMap();
					paraMap.put("msg",msgObj.msg);
					if(RegionUtil.isPc())
						AlertPlugin.showMsg(msgObj.title,380,200,"common/shared/popup/confirm.rs",paraMap);
					else
						AlertPlugin.showMsg(msgObj.title,350,200,"common/shared/popup/appconfirm.rs",paraMap);
					AlertPlugin.msgDef = new jQuery.Deferred();
					AlertPlugin.msgPromise = AlertPlugin.msgDef.promise();
				}
			}
		},
		alert:function(msg,title){
			return RS.toast(msg,2500);
			//return RS.prompt(title,msg,2000);
//			var msgObj = {};
//			msgObj.type= 1;
//			msgObj.msg = msg;
//			msgObj.title = title;
//			msgObj.msgDef = new jQuery.Deferred();
//			msgObj.msgPromise = msgObj.msgDef.promise();
//			AlertPlugin.msgQueue.push(msgObj);
//			if(AlertPlugin.msgPromise==null){
//				AlertPlugin.consumeMsgQueue();
//			}
//			else{
//				AlertPlugin.msgPromise.done(function(){
//					AlertPlugin.consumeMsgQueue();
//				})
//			}
//			return msgObj.msgPromise;
		},
		confirm:function(msg,title,yesCallBack,noCallBack){
			var msgObj = {};
			msgObj.type= 2;
			msgObj.msg = msg;
			msgObj.title = title;
			msgObj.yesCallBack = yesCallBack;
			msgObj.noCallBack = noCallBack;
			msgObj.msgDef = new jQuery.Deferred();
			msgObj.msgPromise = msgObj.msgDef.promise();
			AlertPlugin.msgQueue.push(msgObj);
			if(AlertPlugin.msgPromise==null){
				AlertPlugin.consumeMsgQueue();
			}
			else{
				AlertPlugin.msgPromise.done(function(){
					AlertPlugin.consumeMsgQueue();
				})
			}
			return msgObj.msgPromise;
		},
		yes:function(event){
			if(AlertPlugin.yesCallBack!=null && (typeof AlertPlugin.yesCallBack ==="function")){
				AlertPlugin.yesCallBack();
			}
			AlertPlugin.hideMsg(event,true);
		},
		no:function(event){
			if(AlertPlugin.noCallBack!=null && (typeof AlertPlugin.yesCallBack ==="function")){
				AlertPlugin.noCallBack();
			}
			AlertPlugin.hideMsg(event,false);
		},
//		hideToast:function(toastWindowObject,duration){
//			setTimeout(function(){
//				var animationHolder = toastWindowObject.find(".animation-holder");
//				animationHolder.addClass("moveout-animation2-class");
//				animationHolder.removeClass("movein-animation2-class");
//				//关闭动画结束后回调
//				animationHolder.bind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend', function(){	
//					var promiseId = toastWindowObject.attr("promise-id");
//					var toastDef = window.toastDefs[promiseId];
//					toastDef.resolve();
//					delete window.toastDefs[promiseId];
//					toastWindowObject.remove();
//				})
//				
//			},duration);
//		},
//		toast:function(msg,duration,width,height){
//			var toastDef = new jQuery.Deferred();
//			
//			var promiseId = RegionUtil.getUUID();
//			if(window.toastDefs==null){
//				window.toastDefs={};
//			}
//			window.toastDefs[promiseId] = toastDef;
//			
//			if(duration==null)duration=3000;
//			if(width==null)width = 200;
//			if(height==null)height = 40;
//			
//			var coverMaskDiv=document.createElement("div");
//			coverMaskDiv.className = "hidden cover-container toast-container";
//			var toastWindowObject = $(coverMaskDiv);
//			toastWindowObject.html(AlertPlugin.toastHtml);
//			document.body.appendChild(toastWindowObject[0]);
//			
//			/*$(window).resize(function() {//window resize监听
//				AlertPlugin.placeToast(toastWindowObject.find(".cover-window"),toastWindowObject.width,toastWindowObject.height);	
//			});*/
//			
//			toastWindowObject.find(".cover-window").css("width",width);
//			//toastWindowObject.find(".cover-window").css("height",height);
//			
//			if(msg!=null)toastWindowObject.find(".modal-container").text(msg);
//			
////			toastWindowObject.width = width;
////			toastWindowObject.height = height;
//			toastWindowObject.find(".animation-holder").removeClass("moveout-animation2-class");
//			toastWindowObject.find(".animation-holder").addClass("movein-animation2-class");
//			
//			toastWindowObject.removeClass("hidden");
//			
//			toastWindowObject.attr("promise-id",promiseId);
//			
//			AlertPlugin.placeToast(toastWindowObject.find(".cover-window"),width,height);
//			AlertPlugin.hideToast(toastWindowObject,duration);
//			
//			return toastDef.promise();
//		},//计算toast的位置
//		placeToast:function(divObj,width,height){
//			var windowWidth = $(window).width();
//			var windowHeight = $(window).height();
//			
//			var maxWidth = windowWidth-20;
//			if(width>maxWidth)width=maxWidth;
//		
//			var divWidth = width;
//			var divHeight = height;
//			
//			var divLeft = (windowWidth-divWidth)/2;
//			var divTop = (windowHeight-divHeight)/2;
//			
//			var topPostion = 0.48*windowHeight;
//			
//			divObj.css("left",divLeft);
//			divObj.css("top",topPostion);
//			divObj.css("max-width",divWidth);
//			divObj.css("min-height",divHeight);
//			divObj.css("padding","0px 0px 0px 0px");
//		},
		hideMsg:function(event,result){
			var tmpElem = RegionUtil.getEventTarget(event);
			var msgWindowObject = $(tmpElem).closest(".cover-container");
			msgWindowObject.find(".animation-holder").addClass("moveout-animation2-class");
			msgWindowObject.find(".animation-holder").removeClass("movein-animation2-class");
			RegionUtil.enableScrollAfterPopupClosed();
			
			//关闭动画结束后回调
			msgWindowObject.find(".animation-holder").bind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend', function(){	
				msgWindowObject.remove();
				if(result==null)result=false;
				if(AlertPlugin.msgDef!=null){
					var defTmp = AlertPlugin.msgDef;
					AlertPlugin.msgDef = null;
					AlertPlugin.msgPromise = null;
					defTmp.resolve(result);
					
				}
			})
		},
		showMsg:function(title,width,height,typeRegionUrl,paraMap){
			var coverMaskDiv=document.createElement("div");
			coverMaskDiv.className = "hidden cover-container";
			var msgWindowObject = $(coverMaskDiv);
			msgWindowObject.html(AlertPlugin.alertHtml);
			document.body.appendChild(msgWindowObject[0]);
			
			
			$(window).resize(function() {//window resize监听
				var fontSize = parseInt(document.documentElement.style.fontSize);
				//ddh   msgWindowObject.width = width*fontSize/70;
				//ddh   msgWindowObject.height = height*fontSize/70;
				msgWindowObject.width = fontSize*26;
				msgWindowObject.height = fontSize*14;
				RegionUtil.calPositionWithFixedSize(msgWindowObject.find(".cover-window"),msgWindowObject.width,msgWindowObject.height);
			});
			if(title!=null)msgWindowObject.find(".title").text(title);
			
			var fontSize = parseInt(document.documentElement.style.fontSize);
	
			msgWindowObject.width = fontSize*26;
			msgWindowObject.height = fontSize*14;
			msgWindowObject.find(".animation-holder").removeClass("moveout-animation2-class");
			msgWindowObject.find(".animation-holder").addClass("movein-animation2-class");
			
			msgWindowObject.removeClass("hidden");
			RegionUtil.disableScrollAfterPopupOpened();
			
			RegionUtil.calPositionWithFixedSize(msgWindowObject.find(".cover-window"),msgWindowObject.width,msgWindowObject.height);
			if(typeRegionUrl=="common/shared/popup/alert.rs"){
				RegionUtil.loadRegionByContent(msgWindowObject.find(".modal-container"),null,AlertPlugin.alertRS,paraMap,"REGIONmsgWindow");
			}
			else {
				if(!RegionUtil.isPc()){
					/*msgWindowObject.find(".cover-window").css("width","100%");
					msgWindowObject.find(".cover-window").css("bottom","0");
					msgWindowObject.find(".cover-window").css("left","0");
					msgWindowObject.find(".cover-window").css("top","");*/
				}
				RegionUtil.loadRegion(msgWindowObject.find(".modal-container"),RegionUtil.appendParaMapToUrl(typeRegionUrl,paraMap),"REGIONmsgWindow");
			}
		},
		alertRS:"<div id=\"REGION\" class=\"hidden text-center\">\r\n" + 
		"	<div class=\"alert-content\"><span style=\"position: relative;top: 20%\" region-attr=\"msg\"></span></div>\r\n" + 
		"	<button class=\"btn\" data-type='primary' normal onclick='AlertPlugin.hideMsg(event)'><message key=\"global_msg.ok\"></message></button>\r\n" +
		"</div>	\r\n" + 
		"<script type=\"text/javascript\">\r\n" + 
		"RegionUtil.ready(function(){\r\n" + 
		"	var formRegion = RegionUtil.newFormRegion(\"#REGION\");\r\n" + 
		"	formRegion.renderRegion();\r\n" + 
		"})\r\n" + 
		"</script>",
		alertHtml: "<div class='cover-bgdiv alert-div'>" + 
		"	<div class='cover-window'>" + 
		"	 	<div class='animation-holder'>" + 
		"			<div class='cover-header col-xs-12'>" + 
		"				<div class='title'>" + 
		"				</div>" + 
		"				<div class='pull-right' style='height:100%'>" + 
		"					<span class='fa-solid fa-remove popclose' aria-hidden='true' onclick='AlertPlugin.hideMsg(event,false)'></span>" + 
		"				</div>" + 
		"			</div>" + 
		"			<div class='modal-container-wrap col-xs-12'>" + 
		"				<div class='modal-container scroll-bar2'>" +
		"				</div>" + 
		"			</div>" + 
		"		</div>" + 
		"	</div>" + 
		"</div>" ,
		toastHtml: "<div class='toast-div'>" + 
		"	<div class='cover-window'>" + 
		"	 	<div class='animation-holder'>" + 
		"			<div class='toast-wrap'>" + 
		"				<div class='modal-container scroll-bar2'>" +
		"				</div>" + 
		"			</div>" + 
		"		</div>" + 
		"	</div>" + 
		"</div>" ,
		promptHolderPromise:null,
		getPromptHolderPromise:function(){
			if(AlertPlugin.promptHolderPromise==null){
				var promptHolder=document.createElement("div");
				document.body.appendChild(promptHolder);
				$(promptHolder).addClass("prompt-holder");
				
				AlertPlugin.promptHolderPromise = RegionUtil.loadRegion($(promptHolder),"common/shared/popup/promptHolder.rs");
			}
			
			return AlertPlugin.promptHolderPromise;
		},
		prompt:function(title,msg,duration,icon,rsFile,paraMap){//
			if(duration==null)duration=5000;//毫秒
			if(icon==null)icon='<i class="fa-regular fa-circle-info"></i>';
			if(rsFile==null)rsFile="common/shared/popup/defaultPromptCard.rs";
			if(paraMap==null)paraMap = new HashMap();
			
			paraMap.put("title",title);
			paraMap.put("msg",msg);
			paraMap.put("duration",duration);
			paraMap.put("icon",icon);
			
			var jqueryDef = new jQuery.Deferred();
			
			var promise = AlertPlugin.getPromptHolderPromise();
			promise.done(function(promptHolderRegion){
				var promptDonePromise = promptHolderRegion.prompt(RS.appendParaMapToUrl(rsFile,paraMap));
				promptDonePromise.done(function(){
					jqueryDef.resolve();
				})
			})
			
			return jqueryDef.promise();
		},
		toastHolderPromise:null,
		getToastHolderPromise:function(){
			if(AlertPlugin.toastHolderPromise==null){
				var toastHolder=document.createElement("div");
				document.body.appendChild(toastHolder);
				$(toastHolder).addClass("toast-holder");
				
				AlertPlugin.toastHolderPromise = RegionUtil.loadRegion($(toastHolder),"common/shared/popup/toastHolder.rs");
			}
			
			return AlertPlugin.toastHolderPromise;
		},
		toast:function(msg,duration,icon,rsFile,paraMap){//
			if(duration==null)duration=3000;//毫秒
			if(icon==null)icon='<i class="fa-regular fa-circle-info"></i>';
			if(rsFile==null)rsFile="common/shared/popup/defaultToastCard.rs";
			if(paraMap==null)paraMap = new HashMap();
			
			paraMap.put("msg",msg);
			paraMap.put("duration",duration);
			paraMap.put("icon",icon);
			
			var jqueryDef = new jQuery.Deferred();
			
			var promise = AlertPlugin.getToastHolderPromise();
			promise.done(function(toastHolderRegion){
				var toastDonePromise = toastHolderRegion.toast(RS.appendParaMapToUrl(rsFile,paraMap));
				toastDonePromise.done(function(){
					jqueryDef.resolve();
				})
			})
			
			return jqueryDef.promise();
		}
}