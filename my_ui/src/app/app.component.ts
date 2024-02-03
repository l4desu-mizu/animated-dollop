import { Component, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import {MyType} from 'multilang_playground';

// for web target
//import init, {MyType} from 'multilang_playground';
//await init(); // only possible if topLevelAwait is configured with webpack, otherwise should be called on init
// but this will try to fetch the wasm from a server as it assumes to be a simple web application


@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent implements OnInit {
  title = 'my_ui';
  ngOnInit(){
    const a = new MyType(5, "test");
    console.log(a.do_stuff());
  }
}
