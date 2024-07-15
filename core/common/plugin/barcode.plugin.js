var BarcodePlugin = {
		render:function(region,tmpObj,paraData){
			tmpObj.attr("type","hidden");
			tmpObj.addClass("wrapped");
			tmpObj.addClass("hidden");
			
			var format = tmpObj.attr("format");
			var regionAttr = tmpObj.attr("region-attr");
			var barcodeId = "barcode_"+region.regionId +"_" + regionAttr;
			
			var height = tmpObj.attr("height");
			if(height==null)height="100px";
			if(format==null)format="CODE128";
			
			var hiddenInputHtml = tmpObj.prop("outerHTML");
			
			var replaceHtml = '<div class="barcode-wrapper region-wrapper">'+hiddenInputHtml;
			replaceHtml += '<img id="'+barcodeId+'"/>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);			
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			replaced.attr("barcode-id",barcodeId)
			if(paraData!=null&&paraData!=""){
				var barcode = document.getElementById('barcode');
		        var options = {
		            format:format,
		            displayValue:true,
		            fontSize:16,
		            height:height
		        };
		        replaced.find('#'+barcodeId+'').JsBarcode(paraData,options);
			}
			
			replaced.find(".wrapped").attr("real-val",paraData);
			
			tmpObj = replaced.find(".barcode");
			tmpObj.val(paraData);
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			
			var barcodeId = wrapperElemObj.attr("barcode-id");
			regionElemObj.val(paraData);
			
			var format = regionElemObj.attr("format");
			var height = regionElemObj.attr("height");
			if(height==null)height="100px";
			if(format==null)format="CODE128";
			var options = {
		            format:format,
		            displayValue:true,
		            fontSize:16,
		            height:height
		    };
			
			if(paraData!=null&&paraData!="")wrapperElemObj.find('#'+barcodeId+'').JsBarcode(paraData,options);
			wrapperElemObj.find(".wrapped").attr("real-val",paraData);
		}
}