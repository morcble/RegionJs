/**
 * render pagination controller
 * @param grid
 * @param paginationContainer
 * @param totalAmount
 * @param currentPageNo
 * @param totalPageAmount
 * @param pageSize
 */
PaginationPlugin = {
		renderPaginationControl:function(grid,paginationContainer,totalAmount,currentPageNo,totalPageAmount,pageSize, pageBtnShowAmount){
			if(paginationContainer.length==0)return;
			if(totalAmount==0){
				paginationContainer.find(".control").html("");
				paginationContainer.find(".info").html("");
				return;
			}
			
			if(!RegionUtil.isPc()){
				paginationContainer.attr("showgo","false");
				paginationContainer.attr("showinfo","false");
			}
			
			if(paginationContainer.find(".control").length==0){
				paginationContainer.append("<div class=\"control\"></div>");
			};
			if(paginationContainer.find(".info").length==0){
				paginationContainer.append("<div class=\"info\"></div>");
			};

			var pageBtn = {
				totalPageAmount: parseInt(totalPageAmount),
				currentPageNo: parseInt(currentPageNo),
				pageBtnShowAmount: parseInt(pageBtnShowAmount) ? parseInt(pageBtnShowAmount) : 9,
				pageBtnShowStart: null,
				pageBtnShowEnd: null,
				pageExtendBtnLeftShow: false,
				pageExtendBtnRightShow: false,
				template: $("<button>").addClass("btn btn-info btn-sm")
			};
			
			var content = "<div class=\"btn-group\">";
			content += "<button data-type='default' plain class=\"btn btn-info btn-sm refreshBtn\" onclick=\"RegionUtil.getRegionByEvent(event).refreshRegion()\"><i title='Refresh' class='fa-solid fa-undo'></i></i></button>";
			
			if (pageBtn.totalPageAmount > pageBtn.pageBtnShowAmount) {
				var left = parseInt(pageBtn.pageBtnShowAmount / 2 );
				
				pageBtn.pageBtnShowStart = pageBtn.currentPageNo - left;
				pageBtn.pageBtnShowEnd = pageBtn.pageBtnShowStart + pageBtn.pageBtnShowAmount - 1;
				
				if (pageBtn.pageBtnShowStart < 1) {
					pageBtn.pageBtnShowStart = 1;
					pageBtn.pageBtnShowEnd = pageBtn.pageBtnShowStart + pageBtn.pageBtnShowAmount - 1;
				}
				
				if (pageBtn.pageBtnShowEnd > pageBtn.totalPageAmount) {
					pageBtn.pageBtnShowEnd = pageBtn.totalPageAmount;
					pageBtn.pageBtnShowStart = pageBtn.pageBtnShowEnd - pageBtn.pageBtnShowAmount + 1;
				}
				
				pageBtn.pageExtendBtnLeftShow = true;
				pageBtn.pageExtendBtnRightShow = true;
			} else {
				pageBtn.pageBtnShowStart = 1;
				pageBtn.pageBtnShowEnd = pageBtn.totalPageAmount;
			}
			
			if (pageBtn.pageExtendBtnLeftShow) {
				var button = pageBtn.template.clone().addClass("pageExtendBtn pageExtendBtnUp");
				button.append($("<i>").addClass("fa fa-angle-double-left"));
				content += button.prop("outerHTML");
			}
			
			content += "<button class=\"btn btn-info btn-sm prePageBtn\" title=\"Previous\"><i class=\"fa fa-angle-left\"></i></button>";
			
			for(var i = pageBtn.pageBtnShowStart; i <= pageBtn.pageBtnShowEnd; i++) {
				var button = pageBtn.template.clone().attr("pageNo", i).addClass("pageIndexBtn").text(i);
				
				if (i == pageBtn.currentPageNo) {
					button.addClass("pageIndexBtnCurrent");
					button.attr("disabled",true); 
				}
				content += button.prop("outerHTML");
			}
			
			content += "<button class=\"btn btn-info btn-sm nextPageBtn\" title=\"Next\"><i class=\"fa fa-angle-right\"></i></button>";
			
			if (pageBtn.pageExtendBtnRightShow) {
				var button = pageBtn.template.clone().addClass("pageExtendBtn pageExtendBtnDown");
				button.append($("<i>").addClass("fa fa-angle-double-right"));
				content += button.prop("outerHTML");
			}
			
			grid.requestData.pageNumberBtns = totalPageAmount;
			
			
			content += "</div>";
			
			if(paginationContainer.attr("showpagesize")!="false"){
				content += "<div class=\"pagination-option\">";
				//content += "<span class=\"text-normal\">Page Size</span>";
				content += "<select class=\"btn btn-info btn-sm pageSizeSelect\">";
				content += "<option value=\"10\">10条/页</option>";
				content += "<option value=\"20\">20条/页</option>";
				content += "<option value=\"50\">50条/页</option>";
				content += "<option value=\"100\">100条/页</option>";
				content += "</select>";
				content += "</div>";
			}
			
			
			if(paginationContainer.attr("showgo")!="false"){
				content += "<div class=\"pagination-go\">";
				content += "到第&nbsp;<input type=\"text\" oninput=\"value=value.replace(/[^\\d]/g,'')\" placeholder=\"几\" class=\"form-control input-sm redirectPageInput goPageId\"/>页&nbsp;";
				content += "<button class=\"btn btn-sm redirectPageBtn\" data-type='primary' normal>跳转</button>";
				content += "</div>";
			}
			
			
			paginationContainer.find(".control").html(content);
			var btns = paginationContainer.find(".pageIndexBtn");
			for(var i = 0 ; i <btns.length ; i ++){
				$(btns[i]).click(function(){ 
					grid.gotoPage($(this).attr("pageNo")); 
				});
			}
			
			paginationContainer.find(".pageExtendBtn.pageExtendBtnUp").prop("disabled", parseInt(btns.eq(0).attr("pageNo")) == 1);
			paginationContainer.find(".pageExtendBtn.pageExtendBtnDown").prop("disabled", parseInt(btns.eq(btns.length-1).attr("pageNo")) == pageBtn.totalPageAmount);
			
			
			paginationContainer.find(".pageExtendBtn").click(function() {
				if ($(this).hasClass("pageExtendBtnUp")) {
					var pageTogo = pageBtn.currentPageNo - pageBtn.pageBtnShowAmount;
					pageTogo = pageTogo > 1 ? pageTogo : 1;
					
					grid.gotoPage(pageTogo); 
					
				} else if ($(this).hasClass("pageExtendBtnDown")) {
					var pageTogo = pageBtn.currentPageNo + pageBtn.pageBtnShowAmount;
					pageTogo = pageTogo < pageBtn.totalPageAmount ? pageTogo : pageBtn.totalPageAmount;
					
					grid.gotoPage(pageTogo); 
					
				}
				
				paginationContainer.find(".pageExtendBtn.pageExtendBtnUp").prop("disabled", parseInt(btns.eq(0).attr("pageNo")) == 1);
				paginationContainer.find(".pageExtendBtn.pageExtendBtnDown").prop("disabled", parseInt(btns.eq(btns.length-1).attr("pageNo")) == pageBtn.totalPageAmount);
			});
			
			paginationContainer.find(".pageSizeSelect").val(pageSize); 
			
			if(currentPageNo<=1){
				paginationContainer.find(".prePageBtn").attr("disabled",true); 
			}
			else{
				paginationContainer.find(".prePageBtn").attr("disabled",false); 
			}
			
			if(currentPageNo>=totalPageAmount){
				paginationContainer.find(".nextPageBtn").attr("disabled",true); 
			}
			else{
				paginationContainer.find(".nextPageBtn").attr("disabled",false); 
			}

			paginationContainer.find(".prePageBtn").click(function(){ 
				grid.prePage(); 
			});
			
			paginationContainer.find(".nextPageBtn").click(function(){ 
				grid.nextPage(); 
			});
			
			paginationContainer.find(".redirectPageBtn").click(function(){ 
				var pageNumber = paginationContainer.find(".goPageId").val();
				if(pageNumber==null||pageNumber.trim()==""){
					RegionUtil.alert("Please input page number");
				}
				else{
					grid.gotoPage(pageNumber);
				}
			});
			
			paginationContainer.find(".redirectPageInput").keypress(function(){ 
				if(event.keyCode==13){
					event.preventDefault(); 
					var pageNumber = paginationContainer.find(".goPageId").val();
					if(pageNumber==null||pageNumber.trim()==""){
						RegionUtil.alert("请输入页码");
					}
					else{
						grid.gotoPage(pageNumber);
					}
				}
			});
			
			paginationContainer.find(".pageSizeSelect").change(function(){ 
				grid.requestData.pageSize=$(this).val();
				grid.refreshRegion();
			});
			
			if(paginationContainer.attr("showinfo")!="false"){
				var infoDiv = paginationContainer.find(".info");
				var info ="&nbsp;总共<span class=\"text-normal totalPageDis\">0</span>页,<span class=\"text-normal totalAmountDis\">0</span> <span class=\"text-normal\">条记录</span>";
				infoDiv.html(info);
				
				infoDiv.find(".totalAmountDis").html(totalAmount);
				infoDiv.find(".curPageDis").html(currentPageNo);
				infoDiv.find(".totalPageDis").html(totalPageAmount);
				infoDiv.find(".pageSizeSelect").val(pageSize);
			}
		}
}
