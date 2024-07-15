var RichInputPlugin = {
		render:function(region,tmpObj,paraData){
			tmpObj.attr("type","hidden");
			tmpObj.addClass("wrapped");
			var regionAttr = tmpObj.attr("region-attr");
			var editorId = "editor_"+region.regionId +"_" + regionAttr;
			
			var disabled = tmpObj.hasClass("disabled");
			
			var width = tmpObj.attr("width");
			var height = tmpObj.attr("height");
			var scrolling = tmpObj.attr("scrolling");
			var hiddenInputHtml = tmpObj.prop("outerHTML");
			
			var replaceHtml = '<div class="richinput-wrapper region-wrapper">'+hiddenInputHtml;
			replaceHtml += '<div class="rich-wrapper">';
			replaceHtml+='<div id="'+editorId+'"></div>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			if(width!=null)replaced.css("width",width);
			if(height!=null)replaced.css("height",height);
			replaced.attr("editor-id",editorId);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			if(disabled){
				replaced.addClass("disabled");
			}
			
			if(height.endWith("px")){
				height = height.substring(0,height.length-2);
				height = (parseInt(height)-72)+"px";
			}
			
			CKEDITOR.replace(editorId, {
	            language : 'zh-cn',
	            image_previewText:' ' ,
	            height: height
	        });
			CKEDITOR.instances[editorId].setData(paraData);
			
			CKEDITOR.instances[editorId].scrolling = scrolling;
			
			CKEDITOR.instances[editorId].on( 'change', function( event ) {
				var data = this.getData();
				replaced.find(".richinput").val(data);
			});
			
		/*	var docWidth = 3200;
		    var docEl = window.document.documentElement;
			var clientWidth = docEl.getBoundingClientRect().width;
	        window.fontSize =  Math.max(Math.min(16 * (clientWidth / docWidth), 11.2), 8.55) * 2 + 'px';
			*/
			CKEDITOR.instances[editorId].on("instanceReady", function(){
				var editorWindow = this;
				setTimeout(function(){
					if(CKEDITOR.instances[editorId].scrolling!=null){
						editorWindow.setAttribute("scrolling",CKEDITOR.instances[editorId].scrolling);//??没效果
					}
					editorWindow.document.getBody().setStyle("font-size", "16px");
				})
			});
			
			tmpObj = replaced.find(".richinput");
			tmpObj.val(paraData);
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);
			var editorId = wrapperElemObj.attr("editor-id");
			//CKEDITOR.instances[editorId].setData(paraData);
			setTimeout(function(){
				CKEDITOR.instances[editorId].setData(paraData);//兼容 连续调用setData
			},100);
		}
}