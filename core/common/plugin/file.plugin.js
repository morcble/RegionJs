var FilePlugin = {
		render:function(region,tmpObj,paraData){
			var isDisabled = tmpObj.hasClass("disabled");
			
			tmpObj.addClass("wrapped");
			tmpObj.addClass("hidden");
			tmpObj.attr("type","text");
			var hiddenInputHtml = tmpObj.prop("outerHTML");
			var replaceHtml = '<div class="file-wrapper region-wrapper">'+hiddenInputHtml;
			
			var onUploaded = tmpObj.attr("onUploaded");
			var width = tmpObj.attr("width");
			if(width==null)width="150px";
			
			var exsitfileItemsHtml = "";
			var fileIds = "";
			if(Array.prototype.isPrototypeOf(paraData)){
				for(var i = 0 ; i<paraData.length ;i++){
					if(paraData[i].fileId==null)paraData[i].fileId = paraData[i].id;
					if(fileIds.length>0)fileIds+=","+paraData[i].fileId;
					else fileIds+=paraData[i].fileId;
					var fileName = paraData[i].fileKey.substring(paraData[i].fileKey.lastIndexOf("/")+1,paraData[i].fileKey.length);
					if(isDisabled)
						exsitfileItemsHtml += '<div class="file-view" file-id="'+paraData[i].fileId+'"><div class="file-info" file-url="'+paraData[i].url+'">'+fileName+'</div></div>';
					else
						exsitfileItemsHtml += '<div class="file-view" file-id="'+paraData[i].fileId+'"><div class="file-info" file-url="'+paraData[i].url+'">'+fileName+'</div><div class="delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div></div>'
				}
			}
			
			
			replaceHtml += '<input type="file" class="hidden file-selector">';
			replaceHtml+='	<div class="uploaded-files">'+exsitfileItemsHtml+'<div class="brace"></div></div>';

			//plus-view begin

			replaceHtml+='	<div class="plus-view">';
			replaceHtml+='	<div class="progress-cover hidden"><div class="file-info"></div><div class="progress-block"></div></div>';
			replaceHtml+='	</div>';
			//plus-view end
			if(!isDisabled){
				replaceHtml += '<div class="ctrols"><i class="attach-btn fa fa-plus fa-lg" ></i>';
				replaceHtml += '<i class="cancel-btn hidden fa fa-stop fa-lg" ></i>';
				replaceHtml += '</div>';
			}

			replaceHtml+='</div>';
			var wrapperObj = $(replaceHtml);			
			RegionUtil.addRegionUniqueId(wrapperObj[0],region.regionId);
			tmpObj.replaceWith(wrapperObj);
			tmpObj = wrapperObj.find(".wrapped");
			tmpObj.val(fileIds);
						
			
			var fileObj = wrapperObj.find(".file-selector");

			var fileUploader = new FileUploader(wrapperObj,tmpObj,fileObj,onUploaded);
			wrapperObj.attr("uploader",fileUploader.id);
			
			if(region.uploaders==null){
				region.uploaders = {};
			}
			region.uploaders[fileUploader.id] = fileUploader;
			
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			var isDisabled = regionElemObj.hasClass("disabled");
			wrapperElemObj.find(".uploaded-files>.file-view").remove();
			var fileIds = "";
			if(Array.prototype.isPrototypeOf(paraData) && paraData.length>0){
				var exsitfileItemsHtml = "";
				if(Array.prototype.isPrototypeOf(paraData)){
					for(var i = 0 ; i<paraData.length ;i++){
						if(paraData[i].fileId==null)paraData[i].fileId = paraData[i].id;
						if(fileIds.length>0)fileIds+=","+paraData[i].fileId;
						else fileIds+=paraData[i].fileId;
						var fileName = paraData[i].fileKey.substring(paraData[i].fileKey.lastIndexOf("/")+1,paraData[i].fileKey.length);
						if(isDisabled)
							exsitfileItemsHtml += '<div class="file-view" file-id="'+paraData[i].fileId+'"><div class="file-info" file-url="'+paraData[i].url+'">'+fileName+'</div></div>';
						else
						exsitfileItemsHtml += '<div class="file-view" file-id="'+paraData[i].fileId+'"><div class="file-info" file-url="'+paraData[i].url+'">'+fileName+'</div><div class="delete-btn"><svg xmlns="http://www.w3.org/2000/svg" class="svg-icon" viewBox="0 0 24 24" width="24" height="24"><path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg></div></div>'
					}
				}
				
				RegionUtil.insertBefore(region,wrapperElemObj.find(".uploaded-files>.brace")[0],exsitfileItemsHtml);
				
				var uploadId = wrapperElemObj.attr("uploader")
				region.uploaders[uploadId].initWithExsitData();
			}
			regionElemObj.val(fileIds);
		}
}

