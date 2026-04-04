import { Controller, Get, Post, Put, Delete } from '@structa/runtime';
import { Body, Param } from '@structa/http';
import { UserService } from '../services/user.service';
import { CreateUserDto, UpdateUserDto } from '../dtos/user.dto';

@Controller('/users')
export class UserController {
    constructor(private readonly userService: UserService) {}

    @Get('/')
    async findAll(): Promise<User[]> {
        return this.userService.findAll();
    }

    @Get('/:id')
    async findById(@Param('id') id: string): Promise<User | null> {
        return this.userService.findById(id);
    }

    @Post('/')
    async create(@Body() createUserDto: CreateUserDto): Promise<User> {
        return this.userService.create(createUserDto);
    }

    @Put('/:id')
    async update(
        @Param('id') id: string, 
        @Body() updateUserDto: UpdateUserDto
    ): Promise<User | null> {
        return this.userService.update(id, updateUserDto);
    }

    @Delete('/:id')
    async delete(@Param('id') id: string): Promise<boolean> {
        return this.userService.delete(id);
    }
}
