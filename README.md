# SimpleRestaurantApiClient

<p>This project contains the client application.</p>

<h2>Build & Launch the Application</h2>
<p>This application need to be run after the server one. (https://github.com/DidierNicolas/SimpleRestaurantApiServer)<br>
To build and run this application, apply theses instructions</p>
<code>cargo build --bin SimpleRestaurantApiClient </code><br>
<code>cargo run --bin SimpleRestaurantApiClient [num]</code>

<p><code>[num]</code> is the number of client making requests <br><br>
  For this application one client = one restaurant staff device = one table
</p>

<h2>Outputs</h2>
The application will continusly make http request until you stop it.
It will randomly make an http request between :
<h4>CREATE ITEM</h4>
<p>It will output the content of the created item(s) for the table number<br>
<img width="943" alt="image" src="https://user-images.githubusercontent.com/19792149/168812477-d281db1b-abe5-4aa5-9577-43f83b419693.png">
</p>

<h4>GET ITEM</h4>
<p>It will output the content of an item for an item number <br>
<img width="509" alt="image" src="https://user-images.githubusercontent.com/19792149/168960397-685255ad-a614-4b62-8a47-b818b97bd09b.png"> <br> <br>
  It will output an error if there is no item for an item number for the table<br>
  <img width="420" alt="image" src="https://user-images.githubusercontent.com/19792149/168960084-1848e455-f662-4bf8-9213-f1d590b9509b.png">
</p>

<h4>UPDATE ITEM</h4>
<p>It will output the content of the updated item for an item number <br>
  <img width="522" alt="image" src="https://user-images.githubusercontent.com/19792149/168960728-de91450f-6688-4a74-bcd1-f20cb650f300.png"><br><br>
  It will output an error if there is no item for an item number for the table <br>
  <img width="450" alt="image" src="https://user-images.githubusercontent.com/19792149/168960229-356ddf45-2639-44c7-baf8-70a3d8c5325b.png">
</p>

<h4>DELETE ITEM</h4>
<p>It will output 1 if an item has been deleted. <br>
<img width="186" alt="image" src="https://user-images.githubusercontent.com/19792149/168960640-9f578c59-291f-4c77-bc57-314047c9e242.png"><br> <br> 
It will output 0 if no item has been deleted<br>
<img width="194" alt="image" src="https://user-images.githubusercontent.com/19792149/168959281-f3ed32fc-039a-4c01-b773-6012d8a15c48.png">  
</p>

<h4>LIST ITEMS FOR A TABLE</h4>
<p>It will output the content of all items for a table number <br> 
  <img width="1047" alt="image" src="https://user-images.githubusercontent.com/19792149/168959400-c0029f7f-55c8-4089-a834-ff1b812d9b6d.png"> <br><br>
It will output "[]" if there is no items for this table <br>
  <img width="164" alt="image" src="https://user-images.githubusercontent.com/19792149/168959563-1f4f81bb-7696-4d3e-9492-b7b42acd5bd6.png">
</p>





