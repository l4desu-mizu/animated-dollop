import { Component, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import init, {MyType} from 'multilang_playground';

await init(); // only possible if topLevelAwait is configured with webpack, otherwise should be called on init


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
