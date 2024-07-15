var ImgPlugin = {
		render:function(region,tmpObj,paraData){
			var isDisabled = tmpObj.hasClass("disabled");
			
			tmpObj.addClass("wrapped");
			tmpObj.addClass("hidden");
			tmpObj.attr("type","text");
			var hiddenInputHtml = tmpObj.prop("outerHTML");
			var replaceHtml = '<div class="img-wrapper region-wrapper">'+hiddenInputHtml;
			
			var useCamera = tmpObj.attr("camera");
			
			var onUploaded = tmpObj.attr("onUploaded");
			var width = tmpObj.attr("width");
			if(width==null)width="150px";
			
			var hint = tmpObj.attr("hint");

			var exsitfileItemsHtml = "";
			var fileIds = "";
			if(Array.prototype.isPrototypeOf(paraData)){
				for(var i = 0 ; i<paraData.length ;i++){
					if(paraData[i].fileId==null)paraData[i].fileId = paraData[i].id;
					if(fileIds.length>0)fileIds+=","+paraData[i].fileId;
					else fileIds+=paraData[i].fileId;
					var fileName = paraData[i].fileKey.substring(paraData[i].fileKey.lastIndexOf("/")+1,paraData[i].fileKey.length);
					if(isDisabled)exsitfileItemsHtml += '<div class="img-view" file-id="'+paraData[i].fileId+'"><img class="preview lazyImg" data-original="'+paraData[i].url+'"><div class="delete-btn hidden"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div></div>'
					else exsitfileItemsHtml += '<div class="img-view" file-id="'+paraData[i].fileId+'"><img class="preview lazyImg" data-original="'+paraData[i].url+'"><div class="delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div></div>'
				}
			}
			
		
			if(useCamera=="true") replaceHtml += '<input type="file" capture="camera" accept="image/*" class="hidden file-selector">';
			else  replaceHtml += '<input type="file" accept="image/*" class="hidden file-selector">';

			replaceHtml+='	<div class="uploaded-imgs">'+exsitfileItemsHtml+'<div class="brace"></div></div>';
			//plus-view begin
			if(!isDisabled){
				replaceHtml+='	<div class="plus-view attach-btn">';
				replaceHtml+='	<div class="plus-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 32 32" width="18" height="18" style="fill: rgba(0, 0, 0, 0.45);"><path d="M29.714 15.086h-12.8v-12.8h-2.286v12.8h-12.57v2.286h12.571v12.57h2.286v-12.57h12.8z"></path></svg></div>';
				if(hint!=null){
					replaceHtml+='	<div class="hint">'+hint+'</div>';
				}
				else{
					if(useCamera=="true")
						replaceHtml+='	<div class="hint">拍照</div>';
					else
						replaceHtml+='	<div class="hint">上传照片</div>';
				}
				replaceHtml+='	<div class="progress-cover hidden"><div class="progress-block"></div></div>';
				replaceHtml+='	<div class="img-view hidden"><img class="preview"></div>';
				replaceHtml+='	</div>';
			}
			//plus-view end
			
			replaceHtml += '<div class="ctrols">';
			replaceHtml += '<i class="cancel-btn hidden fa fa-stop fa-lg"></i>';
			replaceHtml += '</div>';
			
			replaceHtml+='</div>';
			var wrapperObj = $(replaceHtml);			
			RegionUtil.addRegionUniqueId(wrapperObj[0],region.regionId);
			tmpObj.replaceWith(wrapperObj);
			tmpObj = wrapperObj.find(".wrapped");
			if(isDisabled)wrapperObj.addClass("disabled");
			
			
			tmpObj.val(fileIds);
			var fileObj = wrapperObj.find(".file-selector");
			
			fileObj.attr("file-type","image")
			var fileUploader = new FileUploader(wrapperObj,tmpObj,fileObj,onUploaded);
			wrapperObj.attr("uploader",fileUploader.id);
			
			if(region.uploaders==null){
				region.uploaders = {};
			}
			region.uploaders[fileUploader.id] = fileUploader;
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			var isDisabled = wrapperElemObj.hasClass("disabled");
			wrapperElemObj.find(".uploaded-imgs>.img-view").remove();
			
			var fileIds = "";
			if(Array.prototype.isPrototypeOf(paraData) && paraData.length>0){
				var exsitfileItemsHtml = "";
				if(Array.prototype.isPrototypeOf(paraData)){
					for(var i = 0 ; i<paraData.length ;i++){
						if(paraData[i].fileId==null)paraData[i].fileId = paraData[i].id;
						if(fileIds.length>0)fileIds+=","+paraData[i].fileId;
						else fileIds+=paraData[i].fileId;
						var fileName = paraData[i].fileKey.substring(paraData[i].fileKey.lastIndexOf("/")+1,paraData[i].fileKey.length);
						if(isDisabled)exsitfileItemsHtml += '<div class="img-view" file-id="'+paraData[i].fileId+'"><img class="preview lazyImg" data-original="'+paraData[i].url+'"><div class="hidden delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div></div>'
						else exsitfileItemsHtml += '<div class="img-view" file-id="'+paraData[i].fileId+'"><img class="preview lazyImg" data-original="'+paraData[i].url+'"><div class="delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div></div>'
					}
				}
				
				RegionUtil.insertBefore(region,wrapperElemObj.find(".uploaded-imgs>.brace")[0],exsitfileItemsHtml);
				
				var uploadId = wrapperElemObj.attr("uploader")
				region.uploaders[uploadId].initWithExsitData();
				
				if(Config.lazyLoadImg&&paraData.length>0){
					window.setTimeout(function(){
						region.find('.lazyImg').lazyload({  
							placeholder : "images/loading.gif",
				        	threshold: 200,  
				        	effect: 'fadeIn',
				        	container: wrapperElemObj,
				   	 	})
					},10);
				}
			}
			
			regionElemObj.val(fileIds);
		},
		view:function(region,tmpObj,paraData){
			/*if(paraData==null||paraData.url==null)return;
			var width = tmpObj.attr("width");
			if(width==null)width="150px";
			
			var height = tmpObj.attr("height");
			if(height==null)height="150px";
			var replaceHtml = '<div class="img-wrapper"><image class="wrapped preview-image" region-attr="name">';
			replaceHtml+='	</div>';
			
			var replaced = $(replaceHtml);	
			
			var imgObj = replaced.find(".wrapped");
			imgObj.css("width",width);
			imgObj.css("height",height);
			imgObj.attr("src",Config.frontendPath+"/images/loading.gif");
			imgObj.addClass("lazyImg");
			imgObj.attr("data-original",paraData.url);
			
			tmpObj.replaceWith(replaced);
			
			window.setTimeout(function(){
				imgObj.lazyload({    
		        	threshold: 200,  
		        	effect: 'fadeIn',
		        	container: $(region.getRegionDivElem()),
		   	 	})
			},10);
			return replaced;*/
		}
}