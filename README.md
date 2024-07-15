# Context And Issue
- 1.When releasing a new version of your Web system, are you already tired of relying on many external libraries and the lengthy compilation process?
- 2.Are you looking for a way to build arbitrarily complex web systems solely relying on JavaScript and CSS technologies only?
- 3.Do you miss the old days when you could simply copy the source files to update and deploy your web system?
- 4.But you still hope to use web components to enhance the readability and reusability of your program.

If you got the above concerns, please try out this web framework. It's a lightweight and extremely easy-to-start web frontend framework, it is named Region JS.

# Overview Of Region JS
- It is a modern web frontend framework constructed with a comprehensive component-oriented approach and adhering to object-oriented principles. It extends standard HTML tags and compatible with jQuery syntax.
- Adhering to conventional component standards, it supports data binding, CSS styling, and JavaScript isolation for components.
- Without the need for compiling source code, it offers a what-you-see-is-what-you-get experience.

# The basic composition structure of a component
```html
<style type="text/css">
	/* component scope css */
</style>

<div id="REGION" class="hidden">
	  <!--  component scope html -->
</div>	

<script type="text/javascript">
var REGION = {
	afterRenderData:function(){
            <!--  component lifecycle of it is renderred or refreshed -->
	}
};

<!--  component mounted trigger -->
RS.ready(function(){
	var region = RS.newRegion("#REGION",null);
	region.afterRenderData = REGION.afterRenderData;
	region.renderRegion();
})
</script>
